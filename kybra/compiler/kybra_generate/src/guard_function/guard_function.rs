use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(function_name: &String) -> TokenStream {
    quote! {
        unsafe {
            // TODO is this a security vulnerability?
            if INTERPRETER_OPTION.is_none() {
                return Ok(());
            }

            let interpreter = INTERPRETER_OPTION
                .as_mut()
                .ok_or_else(|| "SystemError: missing python interpreter".to_string())?;
            let scope = SCOPE_OPTION
                .as_mut()
                .ok_or_else(|| "SystemError: missing python scope".to_string())?;

            interpreter.enter(|vm| {
                scope
                    .globals
                    .get_item(#function_name, vm)
                    .map_err(|err| {
                        let py_object = err.to_pyobject(vm);
                        let type_name = py_object.class().name().to_string();
                        match py_object.str(vm) {
                            Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
                            Err(_) => format!("Attribute Error: '{type_name}' object has no attribute '__str__'"),
                        }
                    })?
                    .call((), vm)
                    .map_err(|err| {
                        let py_object = err.to_pyobject(vm);
                        let type_name = py_object.class().name().to_string();
                        match py_object.str(vm) {
                            Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
                            Err(_) => format!("Attribute Error: '{type_name}' object has no attribute '__str__'"),
                        }
                    })?
                    .try_from_vm_value(vm)
                    // .map_err(|vmc_err| "TypeError: value is not a GuardResult".to_string())?
                    .map_err(|vmc_err| vmc_err.0)?
            })
        }
    }
}
