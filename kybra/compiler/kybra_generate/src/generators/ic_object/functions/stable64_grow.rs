use quote::quote;

pub fn generate_stable64_grow() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn stable64_grow(&self, new_pages_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let new_pages: u64 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::stable::stable64_grow(new_pages).try_into_vm_value(vm).unwrap()
        }
    }
}
