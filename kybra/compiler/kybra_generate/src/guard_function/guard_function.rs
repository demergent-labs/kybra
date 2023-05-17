use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(function_name: &String) -> TokenStream {
    quote! {
        unsafe {
            // TODO is this a security vulnerability?
            if INTERPRETER_OPTION.is_none() {
                return Ok(());
            }

            let interpreter = INTERPRETER_OPTION.as_mut().unwrap();
            let scope = SCOPE_OPTION.as_mut().unwrap();
            interpreter.enter(|vm| {
                let method_py_object_ref =
                    scope.globals.get_item(#function_name, vm).unwrap_or_trap(vm, None);
                let result_py_object_ref = vm.invoke(&method_py_object_ref, ());
                match result_py_object_ref {
                    Ok(py_object_ref) => py_object_ref.try_from_vm_value(vm).unwrap(),
                    Err(err) => {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
                        panic!("{}", err_string);
                    }
                }
            })
        }
    }
}
