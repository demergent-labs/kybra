use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_caller(
            &self,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            ic_cdk::api::caller().try_into_vm_value(vm).unwrap()
        }
    }
}
