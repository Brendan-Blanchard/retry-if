use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parse;
use syn::parse::{ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{ImplItemFn, ItemFn};

// TODO: see https://github.com/stonecodekiller/rate-limit/blob/master/src/lib.rs for arg parsing

// TODO: structure: Can take a struct to wrap the backoff parameters in, then...?
//  1
//      can take arbitrary patterns to match
//      syn::Variant is promising? Hard to tell without implementing
//  2
//      can take some fn to return bool of match or not (maybe best?)
//      if it's an ident, using quote! it's easy to call and ignore the exact type

// TODO: not sure how to parse to one/many of ItemFn, TraitItemFn, etc
//  generic over all would be great...but do I need to even?
//      just need to mimic the signature and return code to match...so probably
//  Yeah: can do: https://stackoverflow.com/questions/57342132/how-to-find-the-correct-return-type-for-synparse

enum Functions {
    Free(ItemFn),
    Impl(ImplItemFn),
}

impl Parse for Functions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO: this always parses to ImplItemFn...?
        if let Ok(impl_fn) = input.parse() {
            Ok(Functions::Impl(impl_fn))
        } else if let Ok(free_fn) = input.parse() {
            Ok(Functions::Free(free_fn))
        } else {
            panic!("failed to parse item under #[retry(...)] as a function or impl method");
        }
    }
}

#[proc_macro_attribute]
pub fn retry(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let comma_punctuated = Punctuated::<syn::Ident, syn::Token![,]>::parse_separated_nonempty;

    let punctuated_args = comma_punctuated.parse(args).unwrap();
    let mut punctuated_args_iter = punctuated_args.iter();

    let config = punctuated_args_iter
        .next()
        .expect("configuration must be supplied as an argument to #[retry(...)]");
    let should_retry = punctuated_args_iter
        .next()
        .expect("should_retry test must be supplied as the second argument to #[retry(...)]");

    eprintln!("{:?}", config);
    eprintln!("{:?}", should_retry);

    let parsed: Functions =
        syn::parse(item.clone()).expect("failed to parse item under #[retry(...)]");

    match parsed {
        Functions::Free(free_fn) => decorate_free_fn(free_fn, config, should_retry),
        Functions::Impl(impl_fn) => decorate_impl_fn(impl_fn, config, should_retry),
    }
}

fn decorate_free_fn(free_fn: ItemFn, config: &Ident, test: &Ident) -> proc_macro::TokenStream {
    let attrs = &free_fn.attrs;
    let vis = &free_fn.vis;
    let sig = &free_fn.sig;
    let block = &free_fn.block;

    (quote! {
        #(#attrs)*
        #vis #sig {
            println!("Free function decorated!");

            #block
        }
    })
    .into()
}

fn decorate_impl_fn(
    impl_fn: ImplItemFn,
    config: &Ident,
    should_retry: &Ident,
) -> proc_macro::TokenStream {
    let attrs = &impl_fn.attrs;
    let vis = &impl_fn.vis;
    let sig = &impl_fn.sig;
    let block = &impl_fn.block;

    (quote! {
        #(#attrs)*
        #vis #sig {
            println!("Impl fn decorated!");

            let mut result = #block;
            let start = tokio::time::Instant::now();

            for attempt in 0..#config.max_tries {
                if !#should_retry(result) {
                    break;
                }

                let retry_wait = #config.t_wait.mul_f64(#config.backoff.powi(attempt)).min(#config.backoff_max.unwrap_or(std::time::Duration::MAX));

                let now = tokio::time::Instant::now();
                let since_start = now - start;
                let will_exceed_time = since_start + retry_wait > #config.t_wait_max;

                if will_exceed_time {
                    break;
                }

                println!("Sleeping: {:?}", retry_wait);
                tokio::time::sleep(retry_wait).await;

                result = #block;
            }

            result
        }
    })
    .into()
}
