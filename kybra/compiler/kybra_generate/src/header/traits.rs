pub fn generate() -> proc_macro2::TokenStream {
    quote::quote! {
        trait ToCdkActTryIntoVmValueError {
            fn to_cdk_act_try_into_vm_value_error(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> CdkActTryIntoVmValueError;
        }
        impl ToCdkActTryIntoVmValueError for rustpython_vm::builtins::PyBaseExceptionRef {
            fn to_cdk_act_try_into_vm_value_error(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> CdkActTryIntoVmValueError {
                let py_object = self.to_pyobject(vm);
                let type_name = py_object.class().name().to_string();
                let err_message = match py_object.str(vm) {
                    Ok(str) => str,
                    Err(_) => {
                        return CdkActTryIntoVmValueError(format!(
                            "Attribute Error: '{}' object has no attribute '__str__'",
                            type_name
                        ));
                    }
                };

                CdkActTryIntoVmValueError(format!("{}: {}", type_name, err_message))
            }
        }

        trait ToRustErrString {
            fn to_rust_err_string(self, vm: &rustpython::vm::VirtualMachine) -> String;
        }

        impl ToRustErrString for rustpython_vm::builtins::PyBaseExceptionRef {
            fn to_rust_err_string(self, vm: &rustpython::vm::VirtualMachine) -> String {
                let py_object = self.to_pyobject(vm);
                let type_name = py_object.class().name().to_string();
                match py_object.str(vm) {
                    Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
                    Err(_) => {
                        format!("Attribute Error: '{type_name}' object has no attribute '__str__'")
                    }
                }
            }
        }
    }
}
