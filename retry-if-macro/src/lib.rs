use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn retry(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let comma_punctuated = Punctuated::<Ident, syn::Token![,]>::parse_separated_nonempty;
    let punctuated_args = comma_punctuated.parse(args).expect(
        "retry macro requires arguments of ExponentialBackoffConfig and 'should retry' function",
    );
    let mut punctuated_args_iter = punctuated_args.iter();

    let config = punctuated_args_iter
        .next()
        .expect("configuration must be supplied as an argument to #[retry(...)]");

    let retry_if = punctuated_args_iter
        .next()
        .expect("retry_if test must be supplied as the second argument to #[retry(...)]");

    let parsed: ItemFn =
        syn::parse(item).expect("failed to parse item under #[retry(...)] as function");

    decorate_fn(parsed, config, retry_if)
}

/// Wrap the underlying implementation with a retry.
///
/// This takes the underlying function as [ItemFn], the backoff configuration (defined in parent
/// crate) as an `&Ident`, and the `&Ident` for the retry function
fn decorate_fn(impl_fn: ItemFn, config: &Ident, retry_if: &Ident) -> proc_macro::TokenStream {
    let attrs = &impl_fn.attrs;
    let vis = &impl_fn.vis;
    let sig = &impl_fn.sig;
    let block = &impl_fn.block;

    (quote! {
        #(#attrs)*
        #vis #sig {
            let start = tokio::time::Instant::now();
            let backoff_max = #config.backoff_max.unwrap_or(std::time::Duration::MAX);
            let max_tries = #config.max_retries;

            let mut result = #block;

            for attempt in 0..max_tries {
                if !#retry_if(result) {
                    break;
                }

                let retry_wait = #config.t_wait
                    .mul_f64(#config.backoff.powi(attempt))
                    .min(backoff_max);

                if let Some(max_wait) = #config.t_wait_max {
                    let now = tokio::time::Instant::now();
                    let since_start = now - start;
                    let will_exceed_time = since_start + retry_wait > max_wait;

                    if will_exceed_time {
                        break;
                    }
                }

                if cfg!(feature = "tracing") {
                    tracing::info!("Sleeping {retry_wait:?} on attempt {attempt}");
                }

                tokio::time::sleep(retry_wait).await;

                result = #block;
            }

            result
        }
    })
    .into()
}
