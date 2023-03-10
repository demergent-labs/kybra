use quote::quote;

pub fn generate() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_stable_bytes(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::stable::stable_bytes().try_into_vm_value(vm).unwrap()
        }
    }
}
