use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_stable64_write(
            &self,
            offset_py_object_ref: rustpython_vm::PyObjectRef,
            buf_vector_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) {
            let offset: u64 = offset_py_object_ref.try_from_vm_value(vm).unwrap();
            let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm).unwrap();
            let buf: &[u8] = &buf_vector[..];
            ic_cdk::api::stable::stable64_write(offset, buf);
        }
    }
}
