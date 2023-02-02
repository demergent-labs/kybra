use cdk_framework::act::node::FunctionGuard;
use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::StmtKind;

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn as_function_guard(&self) -> Option<FunctionGuard> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { name, .. } => {
                let body = self.generate_function_guard_body(name);
                if self.has_params() {
                    todo!("{}", "Function guards can't have parameters")
                }

                Some(FunctionGuard {
                    body,
                    name: name.clone(),
                })
            }
            _ => None,
        }
    }

    fn has_params(&self) -> bool {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { args, .. } => args.args.len() > 0,
            _ => panic!("Unreachable"),
        }
    }

    fn generate_function_guard_body(&self, function_name: &String) -> TokenStream {
        quote! {
            unsafe{
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
}
