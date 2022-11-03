use quote::quote;

pub fn generate_stable_write() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn stable_write(&self, offset_py_object_ref: PyObjectRef, buf_vector_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
            let offset: u32 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
            let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
            let buf: &[u8] = &buf_vector[..];
            ic_cdk::api::stable::stable_write(offset, buf);
        }
    }
}