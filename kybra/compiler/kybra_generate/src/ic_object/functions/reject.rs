use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn reject(
            &self,
            reject_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let reject_message: String = reject_py_object_ref.try_from_vm_value(vm).unwrap_or_trap();
            ic_cdk::api::call::reject(reject_message.as_str()).try_into_vm_value(vm).unwrap_or_trap()
        }
    }
}
