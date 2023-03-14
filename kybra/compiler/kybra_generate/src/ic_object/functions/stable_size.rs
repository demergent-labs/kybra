use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_stable_size(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::stable::stable_size().try_into_vm_value(vm).unwrap()
        }
    }
}
