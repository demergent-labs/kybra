use quote::quote;

pub fn generate_reply_raw() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn reply_raw(&self, buf_vector_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::call::reply_raw(&buf_vector).try_into_vm_value(vm).unwrap()
        }
    }
}
