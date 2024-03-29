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

    let should_retry = punctuated_args_iter
        .next()
        .expect("should_retry test must be supplied as the second argument to #[retry(...)]");

    let parsed: ItemFn =
        syn::parse(item).expect("failed to parse item under #[retry(...)] as function");

    decorate_fn(parsed, config, should_retry)
}

fn decorate_fn(impl_fn: ItemFn, config: &Ident, should_retry: &Ident) -> proc_macro::TokenStream {
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
                if !#should_retry(result) {
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

                // TODO: add tracing via a cfg/feature?
                println!("Sleeping: {:?}", retry_wait);
                tokio::time::sleep(retry_wait).await;

                result = #block;
            }

            result
        }
    })
    .into()
}
