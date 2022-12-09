use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_tuple(expressions: &Vec<TokenStream>) -> TokenStream {
    let comma = if expressions.len() == 1 {
        quote! { , }
    } else {
        quote! {}
    };
    quote! { (#(#expressions),*#comma) }
}
