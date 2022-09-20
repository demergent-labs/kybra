use proc_macro2::TokenStream;

use super::ToTokenStream;
use crate::cdk_act::nodes::canister_method::CanisterMethodActNode;

pub struct AbstractCanisterTree {
    pub query_methods: Vec<CanisterMethodActNode>,
    pub update_methods: Vec<CanisterMethodActNode>,
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> TokenStream {
        let query_methods: Vec<TokenStream> = self
            .query_methods
            .iter()
            .map(|query_method| query_method.to_token_stream())
            .collect();
        let update_methods: Vec<TokenStream> = self
            .update_methods
            .iter()
            .map(|update_method| update_method.to_token_stream())
            .collect();

        quote::quote! {
            #(#query_methods)*
            #(#update_methods)*
        }
    }
}
