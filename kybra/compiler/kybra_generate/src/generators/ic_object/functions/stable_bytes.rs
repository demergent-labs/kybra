use quote::quote;

pub fn generate_stable_bytes() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn stable_bytes(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::stable::stable_bytes().try_into_vm_value(vm).unwrap()
        }
    }
}
