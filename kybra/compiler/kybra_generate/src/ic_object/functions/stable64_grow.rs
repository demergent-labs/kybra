use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn stable64_grow(
            &self,
            new_pages_py_object_ref:
            rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let new_pages: u64 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap_or_trap();
            ic_cdk::api::stable::stable64_grow(new_pages).try_into_vm_value(vm).unwrap_or_trap()
        }
    }
}
