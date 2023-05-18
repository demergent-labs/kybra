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
                .unwrap_or_trap("SystemError: missing python interpreter");
            let scope = SCOPE_OPTION
                .as_mut()
                .unwrap_or_trap("SystemError: missing python scope");
            interpreter.enter(|vm| {
                let method_py_object_ref =
                    scope.globals.get_item(#function_name, vm).unwrap_or_trap(vm);
                let result_py_object_ref = vm.invoke(&method_py_object_ref, ());
                match result_py_object_ref {
                    Ok(py_object_ref) => py_object_ref.try_from_vm_value(vm).unwrap_or_trap(),
                    Err(err) => {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
                        panic!("{}", err_string);
                    }
                }
            })
        }
    }
}
