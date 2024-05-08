use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{ItemFn, parse_quote, Expr};
use syn::visit_mut;
use syn::visit_mut::VisitMut;

struct BlockModifier;

impl VisitMut for BlockModifier {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        if let Expr::Try(expr_try) = i {
            let expr = &expr_try.expr;
            *i = Expr::Match(parse_quote! {
                match #expr {
                    Ok(val) => val,
                    Err(err) => break 'block Err(err),
                }
            });
        } else if let Expr::Return(expr_return) = i {
            let return_value = &expr_return.expr;
            *i = Expr::Break(parse_quote! {
                break 'block #return_value
            });
        }

        // Important: continue visiting to find nested expressions
        visit_mut::visit_expr_mut(self, i);
    }
}

/// Decorate a function with a given retry configuration.
///
/// Takes two arguments
/// - `ExponentialBackoffConfig`: type defined in parent crate that configures how to back off
/// - retry-if: a predicate that takes the same type as the output of the decorated function
///
/// # Example: Retrying a Result-producing Function on Err(...)
/// The below example sets up a basic retry configuration that will retry up to five times, waiting
/// at first 1 second, then 2 seconds, 4 seconds, etc.
///
/// There is no configured maximum time to retry across all attempts (t_wait_max), nor is there any
/// maximum waiting time on each backoff (backoff_max).
///
/// ```no_run
/// const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
///     max_retries: 5,
///     t_wait: Duration::from_secs(1),
///     backoff: 2.0,
///     t_wait_max: None,
///     backoff_max: None,
/// };
///
/// // this takes an address of the same type as the output of the decorated function
/// //  it returns a boolean specifying if the function should be retried based on the result
/// fn retry_if(result: &Result<i64, TryFromIntError>) -> bool {
///     result.is_err()
/// }
///
/// #[retry(BACKOFF_CONFIG, retry_if)]
/// async fn fallible_call() -> Result<i64, TryFromIntError> {
///     // this will always produce a TryFromIntError, triggering a retry
///     i64::try_from(i128::MAX)
/// }
/// ```
#[proc_macro_attribute]
pub fn retry(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let comma_punctuated = Punctuated::<Ident, syn::Token![,]>::parse_separated_nonempty;
    let punctuated_args = comma_punctuated.parse(args).expect(
        "retry macro requires arguments of ExponentialBackoffConfig and 'retry-if' function",
    );
    let mut punctuated_args_iter = punctuated_args.iter();

    let config = punctuated_args_iter
        .next()
        .expect("configuration must be supplied as an argument to #[retry(...)]");

    let retry_if = punctuated_args_iter
        .next()
        .expect("retry_if predicate must be supplied as the second argument to #[retry(...)]");

    let parsed: ItemFn =
        syn::parse(item).expect("failed to parse item under #[retry(...)] as function");

    decorate_fn(parsed, config, retry_if)
}

/// Wrap the underlying implementation with a retry.
///
/// This takes the underlying function as [ItemFn], the backoff configuration (defined in parent
/// crate) as an `&Ident`, and the `&Ident` for the retry function
fn decorate_fn(mut impl_fn: ItemFn, config: &Ident, retry_if: &Ident) -> proc_macro::TokenStream {
    let attrs = &impl_fn.attrs;
    let vis = &impl_fn.vis;
    let sig = &impl_fn.sig;

    (BlockModifier {}).visit_block_mut(&mut impl_fn.block);
    let block = &impl_fn.block;

    (quote! {
        #(#attrs)*
        #vis #sig {
            let start = tokio::time::Instant::now();
            let backoff_max = #config.backoff_max.unwrap_or(std::time::Duration::MAX);
            let mut attempt = 0;

            loop {
                let result = 'block: {
                    #block
                };

                // Return result if retry isn't required, or if we ran out of attempts
                if !#retry_if(&result) || attempt >= #config.max_retries {
                    return result;
                }
                attempt += 1;

                let retry_wait = #config.t_wait
                    .mul_f64(#config.backoff.powi(attempt))
                    .min(backoff_max);

                if let Some(max_wait) = #config.t_wait_max {
                    let now = tokio::time::Instant::now();
                    let since_start = now - start;

                    // Return if our overall duration is going to exceed `max_wait`
                    if since_start + retry_wait > max_wait {
                        return result;
                    }
                }

                if cfg!(feature = "tracing") {
                    tracing::info!("Sleeping {retry_wait:?} on attempt {attempt}");
                }
                tokio::time::sleep(retry_wait).await;
            }
        }
    })
        .into()
}
