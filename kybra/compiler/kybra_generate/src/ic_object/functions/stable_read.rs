use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn stable_read(
            &self,
            offset_py_object_ref: rustpython_vm::PyObjectRef,
            length_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let offset: u32 = offset_py_object_ref.try_from_vm_value(vm)?;
            let length: u32 = length_py_object_ref.try_from_vm_value(vm)?;
            let mut buf: Vec<u8> = vec![0; length as usize];

            ic_cdk::api::stable::stable_read(offset, &mut buf);

            buf.try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
