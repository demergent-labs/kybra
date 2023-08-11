use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn reply_raw(
            &self,
            buf_vector_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm)?;

            ic_cdk::api::call::reply_raw(&buf_vector)
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
