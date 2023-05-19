use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn candid_encode(
            &self,
            candid_string_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyResult {
            let candid_string: String =
                candid_string_py_object_ref
                    .try_from_vm_value(vm)
                    .map_err(|try_from_err| vm.new_type_error(try_from_err.0))?;

            let candid_args: candid::IDLArgs =
                candid_string
                    .parse::<candid::IDLArgs>()
                    // TODO: We need to create a new CandidError exception class
                    // (and likely subclasses) so that the python code can "except" them
                    // correctly.
                    .map_err(|candid_error| {
                        let exception_type = vm.ctx.exceptions.exception_type.to_owned();
                        vm.new_exception_msg(exception_type, candid_error.to_string())
                    })?;

            let candid_encoded: Vec<u8> =
                candid_args
                    .to_bytes()
                    // TODO: We need to create a new CandidError exception class
                    // (and likely subclasses) so that the python code can "except" them
                    // correctly.
                    .map_err(|candid_error| {
                        let exception_type = vm.ctx.exceptions.exception_type.to_owned();
                        vm.new_exception_msg(exception_type, candid_error.to_string())
                    })?;

            candid_encoded.try_into_vm_value(vm)
                .map_err(|try_from_err| vm.new_type_error(try_from_err.0))
        }
    }
}
