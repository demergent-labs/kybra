use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn trap(
            &self,
            message_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) {
            let message: String = message_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::trap(&message);
        }
    }
}
