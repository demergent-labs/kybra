use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(function_name: &String) -> TokenStream {
    quote! {
        call_global_python_function_sync(#function_name, ())?
    }
}
