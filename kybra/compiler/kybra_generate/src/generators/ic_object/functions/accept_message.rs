use quote::quote;

pub fn generate_accept_message() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn accept_message(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::accept_message().try_into_vm_value(vm).unwrap()
        }
    }
}
