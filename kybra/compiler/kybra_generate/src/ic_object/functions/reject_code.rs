use quote::quote;

pub fn generate() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_reject_code(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::reject_code().try_into_vm_value(vm).unwrap()
        }
    }
}
