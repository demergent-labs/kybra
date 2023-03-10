use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_method_name(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::method_name().try_into_vm_value(vm).unwrap()
        }
    }
}
