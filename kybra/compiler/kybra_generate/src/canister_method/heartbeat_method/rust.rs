use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub fn generate(
    heartbeat_function_def: &SourceMapped<&Located<StmtKind>>,
) -> KybraResult<TokenStream> {
    let function_name = heartbeat_function_def.get_function_name()?;

    Ok(quote! {
        unsafe {
            ic_cdk::spawn(async {
                let vm_interpreter = VM_INTERPRETER_OPTION.as_mut().unwrap();
                let vm_scope = VM_SCOPE.as_mut().unwrap();

                let vm = &vm_interpreter.vm;

                let method_py_object_ref = unwrap_rust_python_result(vm_scope.globals.get_item(#function_name, vm), vm);

                let result_py_object_ref = vm.invoke(&method_py_object_ref, ());

                match result_py_object_ref {
                    Ok(py_object_ref) => async_result_handler(vm, &py_object_ref, vm.ctx.none()).await,
                    Err(err) => {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                        panic!("{}", err_string);
                    }
                };
            });
        }
    })
}
