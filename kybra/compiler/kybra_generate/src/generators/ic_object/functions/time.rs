use quote::quote;

pub fn generate_time() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn time(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::time().try_into_vm_value(vm).unwrap()
        }
    }
}
