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
                let py_object_ref = method_py_object_ref.call((), vm).unwrap_or_trap(vm);

                py_object_ref.try_from_vm_value(vm).unwrap_or_trap()
            })
        }
    }
}
