use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(function_name: &String) -> TokenStream {
    quote! {
        unsafe {
            // TODO is this a security vulnerability?
            if VM_INTERPRETER_OPTION.is_none() {
                return Ok(());
            }

            let vm_interpreter = VM_INTERPRETER_OPTION.as_mut().unwrap();
            let vm_scope = VM_SCOPE.as_mut().unwrap();
            vm_interpreter.enter(|vm| {
                let method_py_object_ref =
                    unwrap_rust_python_result(vm_scope.globals.get_item(#function_name, vm), vm);
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
