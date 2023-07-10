use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[ic_cdk_macros::query]
        fn __get_candid_interface_tmp_hack() -> String {
            __export_service()
        }
    }
}
