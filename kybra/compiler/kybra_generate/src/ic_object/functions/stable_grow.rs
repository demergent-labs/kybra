use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn stable_grow(
            &self,
            new_pages_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let new_pages: u32 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::stable::stable_grow(new_pages).try_into_vm_value(vm).unwrap()
        }
    }
}
