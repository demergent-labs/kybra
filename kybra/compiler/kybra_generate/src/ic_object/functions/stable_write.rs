use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn stable_write(
            &self,
            offset_py_object_ref: rustpython_vm::PyObjectRef,
            buf_vector_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let offset: u32 = offset_py_object_ref.try_from_vm_value(vm)?;
            let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm)?;
            let buf: &[u8] = &buf_vector[..];

            ic_cdk::api::stable::stable_write(offset, buf)
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
