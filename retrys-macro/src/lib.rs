// TODO: see https://github.com/stonecodekiller/rate-limit/blob/master/src/lib.rs for arg parsing

// TODO: not sure how to parse to one/many of ItemFn, TraitItemFn, etc
//  generic over all would be great...but do I need to even?
//      just need to mimic the signature and return code to match...so probably
//  Yeah: can do: https://stackoverflow.com/questions/57342132/how-to-find-the-correct-return-type-for-synparse

#[proc_macro_attribute]
pub fn retry(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}