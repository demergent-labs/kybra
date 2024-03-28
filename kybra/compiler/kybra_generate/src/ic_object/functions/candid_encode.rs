use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn candid_encode(
            &self,
            candid_string_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let candid_string: String = candid_string_py_object_ref.try_from_vm_value(vm)?;

            let candid_args = candid_parser::parse_idl_args(&candid_string)
                .map_err(|candid_error| CandidError::new(vm, candid_error.to_string()))?;

            let candid_encoded: Vec<u8> = candid_args
                .to_bytes()
                .map_err(|candid_error| CandidError::new(vm, candid_error.to_string()))?;

            candid_encoded
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
