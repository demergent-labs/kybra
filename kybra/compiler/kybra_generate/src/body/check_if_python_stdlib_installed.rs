use proc_macro2::TokenStream;

pub fn generate(kybra_version: &str) -> TokenStream {
    quote::quote! {
        #[ic_cdk_macros::query(guard="guard_against_non_controllers", hidden = true)]
        pub fn _kybra_check_if_python_stdlib_installed(kybra_version: String) -> bool {
            kybra_version == #kybra_version && PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| python_stdlib_stable_ref_cell.borrow().get().len() > 0)
        }
    }
}
