use quote::quote;

pub fn generate_stable64_read() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn stable64_read(&self, offset_py_object_ref: PyObjectRef, length_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
            let length: u64 = length_py_object_ref.try_from_vm_value(vm).unwrap();

            let mut buf: Vec<u8> = vec![0; length as usize];
            ic_cdk::api::stable::stable64_read(offset, &mut buf);
            buf.try_into_vm_value(vm).unwrap()
        }
    }
}
