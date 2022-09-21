use proc_macro2::TokenStream;

use super::ToTokenStream;
use crate::cdk_act::nodes::canister_method::CanisterMethodActNode;

pub struct AbstractCanisterTree {
    pub randomness_implementation: TokenStream,
    pub candid_file_generation: TokenStream,
    pub query_methods: Vec<CanisterMethodActNode>,
    pub update_methods: Vec<CanisterMethodActNode>,
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> TokenStream {
        let randomness_implementation = &self.randomness_implementation;
        let candid_file_generation = &self.candid_file_generation;

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
            #randomness_implementation

            #(#query_methods)*
            #(#update_methods)*

            #candid_file_generation
        }
    }

    pub fn generate_randomness_implementation() -> TokenStream {
        quote::quote! {
            fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> { Ok(()) }

            getrandom::register_custom_getrandom!(custom_getrandom);
        }
    }

    pub fn generate_candid_file_generation(did_path: &str) -> TokenStream {
        quote::quote! {
            candid::export_service!();

            #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
            fn export_candid() -> String {
                __export_service()
            }

            #[cfg(test)]

            mod tests {
                use super::*;

                #[test]
                fn write_candid_to_disk() {
                    std::fs::write(#did_path, export_candid()).unwrap();
                }
            }
        }
    }
}
