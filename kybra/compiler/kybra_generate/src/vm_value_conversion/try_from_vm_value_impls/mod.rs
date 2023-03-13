mod basic;
mod generic;
mod numeric;
mod vec;

use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    let basic_impls = basic::generate();
    let generic_impls = generic::generate();
    let numeric_impls = numeric::generate();
    let vec_impls = vec::generate();

    quote::quote! {
        #basic_impls
        #generic_impls
        #numeric_impls
        #vec_impls
    }
}
