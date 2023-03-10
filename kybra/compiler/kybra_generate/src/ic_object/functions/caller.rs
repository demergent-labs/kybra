use quote::quote;

pub fn generate() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_caller(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::caller().try_into_vm_value(vm).unwrap()
        }
    }
}
