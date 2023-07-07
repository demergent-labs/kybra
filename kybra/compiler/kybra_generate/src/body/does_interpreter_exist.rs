pub fn generate() -> proc_macro2::TokenStream {
    quote::quote! {
        #[ic_cdk_macros::query]
        pub fn does_interpreter_exist() -> bool {
            if unsafe { INTERPRETER_OPTION.as_mut() }.is_none() {
                false
            }
            else {
                true
            }
        }
    }
}
