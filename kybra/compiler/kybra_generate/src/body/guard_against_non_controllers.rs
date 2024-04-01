use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        pub fn guard_against_non_controllers() -> Result<(), String> {
            if ic_cdk::api::is_controller(&ic_cdk::api::caller()) {
                Ok(())
            }
            else {
                Err("Not Authorized: only controllers of this canister may call this method".to_string())
            }
        }
    }
}
