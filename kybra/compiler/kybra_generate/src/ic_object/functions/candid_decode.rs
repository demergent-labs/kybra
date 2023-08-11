use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn candid_decode(
            &self,
            candid_encoded_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let candid_encoded: Vec<u8> = candid_encoded_py_object_ref.try_from_vm_value(vm)?;

            let candid_args = candid::IDLArgs::from_bytes(&candid_encoded)
                .map_err(|candid_error| CandidError::new(vm, candid_error.to_string()))?;

            candid_args
                .to_string()
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
