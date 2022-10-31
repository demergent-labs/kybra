use quote::quote;

pub fn generate_stable64_size() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn stable64_size(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::stable::stable64_size().try_into_vm_value(vm).unwrap()
        }
    }
}
