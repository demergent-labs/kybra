use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_accept_message(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::accept_message().try_into_vm_value(vm).unwrap()
        }
    }
}
