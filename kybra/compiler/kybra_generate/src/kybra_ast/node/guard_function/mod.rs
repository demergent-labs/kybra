use std::collections::HashSet;

use cdk_framework::act::node::GuardFunction;
use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, Mod, StmtKind};

use crate::{kybra_ast::NewPyAst, source_map::SourceMapped};

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_function_guard(&self) -> Option<GuardFunction> {
        match &self.node.node {
            StmtKind::FunctionDef { name, .. } => {
                let body = self.generate_function_guard_body(name);
                if self.has_params() {
                    todo!("{}", "Function guards can't have parameters")
                }

                Some(GuardFunction {
                    body,
                    name: name.clone(),
                })
            }
            _ => None,
        }
    }

    fn has_params(&self) -> bool {
        match &self.node.node {
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

impl SourceMapped<Mod> {
    fn get_guard_function_names(&self) -> Vec<String> {
        let guard_function_names: HashSet<_> = match &self.node {
            Mod::Module { body, .. } => body
                .iter()
                .filter_map(|stmt_kind| {
                    SourceMapped {
                        node: stmt_kind,
                        source_map: self.source_map.clone(),
                    }
                    .get_guard_function_name()
                })
                .collect(),
            _ => HashSet::new(),
        };
        guard_function_names.iter().cloned().collect()
    }
}

impl NewPyAst {
    pub fn get_guard_function_names(&self) -> Vec<String> {
        let guard_function_names =
            self.programs
                .iter()
                .fold(HashSet::new(), |mut acc, kybra_program| {
                    acc.extend(kybra_program.get_guard_function_names());
                    acc
                });
        guard_function_names.iter().cloned().collect()
    }
}
