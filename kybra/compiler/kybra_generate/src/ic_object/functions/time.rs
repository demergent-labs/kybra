use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_time(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::time().try_into_vm_value(vm).unwrap()
        }
    }
}
