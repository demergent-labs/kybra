// TODO: There has got to be a better way to grab custom errors.
// Let's figure out how and then swap out all this code.

pub fn generate() -> proc_macro2::TokenStream {
    quote::quote! {
        struct KybraError {}

        impl KybraError {
            fn new(
                vm: &rustpython_vm::VirtualMachine,
                message: String,
            ) -> rustpython_vm::builtins::PyBaseExceptionRef {
                KybraError::subtype(vm, "Error", message)
            }

            fn subtype(
                vm: &rustpython_vm::VirtualMachine,
                subtype: &str,
                message: String,
            ) -> rustpython_vm::builtins::PyBaseExceptionRef {
                let kybra_error_class = match vm
                    .run_block_expr(
                        vm.new_scope_with_builtins(),
                        format!("from kybra import {subtype}; {subtype}").as_str(),
                    ) {
                        Ok(kybra_error_class) => kybra_error_class,
                        Err(py_base_exception) => return py_base_exception
                    };

                let py_type_ref =
                    match rustpython_vm::builtins::PyTypeRef::try_from_object(vm, kybra_error_class)
                    {
                        Ok(py_type_ref) => py_type_ref,
                        Err(py_base_exception) => return py_base_exception
                    };

                vm.new_exception_msg(py_type_ref, message)
            }
        }

        struct CandidError {}

        impl CandidError {
            fn new(
                vm: &rustpython_vm::VirtualMachine,
                message: String,
            ) -> rustpython_vm::builtins::PyBaseExceptionRef {
                KybraError::subtype(vm, "CandidError", message)
            }
        }
    }
}
