use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_id(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyObjectRef {
            ic_cdk::api::id().try_into_vm_value(vm).unwrap()
        }
    }
}
