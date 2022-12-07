use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_tuple(idents: &Vec<Ident>) -> TokenStream {
    let comma = if idents.len() == 1 {
        quote! { , }
    } else {
        quote! {}
    };
    quote! { (#(#idents),*#comma) }
}
