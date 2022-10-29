use quote::quote;

pub fn generate_stable_grow() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn stable_grow(&self, new_pages_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let new_pages: u32 = new_pages_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::stable::stable_grow(new_pages).try_into_vm_value(vm).unwrap()
        }
    }
}
