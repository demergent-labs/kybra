use proc_macro2::TokenStream;
use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMapped;

pub fn generate(heartbeat_function_def: &SourceMapped<&Located<StmtKind>>) -> TokenStream {
    let function_name = heartbeat_function_def.get_function_name();

    quote::quote! {
        unsafe {
            ic_cdk::spawn(async {
                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                let vm = &_kybra_interpreter.vm;

                let method_py_object_ref = _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item(#function_name, vm), vm);

                let result_py_object_ref = vm.invoke(&method_py_object_ref, ());

                match result_py_object_ref {
                    Ok(py_object_ref) => _kybra_async_result_handler(vm, &py_object_ref, vm.ctx.none()).await,
                    Err(err) => {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                        panic!("{}", err_string);
                    }
                };
            });
        }
    }
}
