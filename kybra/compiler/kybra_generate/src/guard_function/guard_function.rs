use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(function_name: &String) -> TokenStream {
    quote! {
        // TODO is this a security vulnerability?
        if unsafe { INTERPRETER_OPTION.is_none() } {
            return Ok(());
        }

        call_global_python_function_sync(#function_name, ())?
    }
}
