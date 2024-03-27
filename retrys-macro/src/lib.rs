use quote::quote;
use syn::punctuated::Punctuated;
use syn::parse::{Parser, ParseStream};
use syn::parse::Parse;
use syn::{ItemFn, TraitItemFn, ImplItemFn};

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
    Trait(TraitItemFn),
    Impl(ImplItemFn),
}

impl Parse for Functions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(free_fn) = input.parse() {
            Ok(Functions::Free(free_fn))
        } else if let Ok(trait_fn) = input.parse() {
            Ok(Functions::Trait(trait_fn))
        } else if let Ok(impl_fn) = input.parse() {
            Ok(Functions::Impl(impl_fn))
        } else {
            panic!("failure!");
        }
    }
}

#[proc_macro_attribute]
pub fn retry(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let comma_punctuated = Punctuated::<syn::Ident, syn::Token![,]>::parse_separated_nonempty;

    let punctuated_args = comma_punctuated.parse(args).unwrap();
    let mut punctuated_args_iter = punctuated_args.iter();

    let variant = punctuated_args_iter.next().unwrap();

    eprintln!("{:?}", variant);

    let parsed: Functions = syn::parse(item.clone()).expect("failed to parse input");

    match parsed {
        Functions::Free(free_fn) => decorate_free_fn(free_fn),
        Functions::Trait(_) => item,
        Functions::Impl(_) => item,
    }
}

fn decorate_free_fn(free_fn: ItemFn) -> proc_macro::TokenStream {
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
    }).into()
}