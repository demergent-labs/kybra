use quote::quote;

pub fn generate_reject_message() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_reject_message(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::reject_message().try_into_vm_value(vm).unwrap()
        }
    }
}
