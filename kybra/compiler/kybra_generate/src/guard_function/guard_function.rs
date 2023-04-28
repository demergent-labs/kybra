use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(function_name: &String) -> TokenStream {
    quote! {
        unsafe {
            // TODO is this a security vulnerability?
            if _KYBRA_INTERPRETER_OPTION.is_none() {
                return Ok(());
            }

            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
            _kybra_interpreter.enter(|vm| {
                let method_py_object_ref =
                    _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item(#function_name, vm), vm);
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
