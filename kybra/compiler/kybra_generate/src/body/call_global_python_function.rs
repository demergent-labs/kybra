use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        async fn call_global_python_function<'a, T>(
            function_name: &str,
            args: impl _KybraTraitIntoFuncArgs
        ) -> Result<T, String>
            where
                for<'b> rustpython::vm::PyObjectRef:
                    CdkActTryFromVmValue<T, &'b rustpython::vm::VirtualMachine>
        {
            unsafe {
                let interpreter = INTERPRETER_OPTION
                    .as_mut()
                    .ok_or_else(|| "SystemError: missing python interpreter")?;
                let scope = SCOPE_OPTION
                    .as_mut()
                    .ok_or_else(|| "SystemError: missing python scope")?;
                let vm = &interpreter.vm;
                let py_object_ref = scope
                    .globals
                    .get_item(function_name, vm)
                    // TODO: Abstract this error conversion logic
                    .map_err(|py_base_exception| {
                        let py_object = py_base_exception.to_pyobject(vm);
                        let type_name = py_object.class().name().to_string();
                        match py_object.str(vm) {
                            Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
                            Err(_) => format!("Attribute Error: '{type_name}' object has no attribute '__str__'"),
                        }
                    })?
                    .call(args, vm)
                    // TODO: Abstract this error conversion logic
                    .map_err(|py_base_exception| {
                        let py_object = py_base_exception.to_pyobject(vm);
                        let type_name = py_object.class().name().to_string();
                        match py_object.str(vm) {
                            Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
                            Err(_) => format!("Attribute Error: '{type_name}' object has no attribute '__str__'"),
                        }
                    })?;
                let final_return_value = async_result_handler(vm, &py_object_ref, vm.ctx.none())
                    .await
                    // TODO: Abstract this error conversion logic
                    .map_err(|py_base_exception| {
                        let py_object = py_base_exception.to_pyobject(vm);
                        let type_name = py_object.class().name().to_string();
                        match py_object.str(vm) {
                            Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
                            Err(_) => format!("Attribute Error: '{type_name}' object has no attribute '__str__'"),
                        }
                    })?;

                final_return_value
                    .try_from_vm_value(vm)
                    .map_err(|vmc_err| vmc_err.0)
            }
        }
    }
}
