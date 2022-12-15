use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::KybraStmt;
use crate::{
    generators::tuple,
    py_ast::{kybra_types::KybraExpr, what_is_it::WhatIsIt},
};
use cdk_framework::{nodes::ActFnParam, ToActDataType};

mod errors;

impl KybraStmt<'_> {
    pub fn build_params(&self) -> Vec<ActFnParam> {
        self.stmt_kind.what_is_it();
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { args, .. } => {
                for arg in &args.args {
                    arg.what_is_it()
                }
                args.args
                    .iter()
                    .map(|arg| {
                        let data_type = match &arg.node.annotation {
                            Some(annotation) => KybraExpr {
                                located_expr: &annotation,
                                source_map: self.source_map.clone(),
                            }
                            .to_act_data_type(&None),
                            None => panic!("{}", self.missing_type_annotation_error()),
                        };
                        ActFnParam {
                            name: arg.node.arg.clone(),
                            data_type,
                        }
                    })
                    .collect()
            }
            _ => panic!("{}", self.not_a_function_def_error()),
        }
    }

    pub fn get_function_name(&self) -> String {
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { name, .. } => name.clone(),
            _ => panic!("{}", self.not_a_function_def_error()),
        }
    }

    pub fn generate_call_to_py_function(&self) -> TokenStream {
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { .. } => {
                let function_name = self.get_function_name();
                let act_params = self.build_params();

                let param_conversions = act_params
                    .iter()
                    .map(|act_fn_param| {
                        let name = format_ident!("{}", act_fn_param.prefixed_name());
                        quote! {
                            #name.try_into_vm_value(vm).unwrap()
                        }
                    })
                    .collect();
                let params = tuple::generate_tuple(&param_conversions);

                quote! {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                    _kybra_interpreter.enter(|vm| {
                        let method_py_object_ref = _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item(#function_name, vm), vm);

                        let result_py_object_ref = vm.invoke(&method_py_object_ref, #params);

                        match result_py_object_ref {
                            Ok(py_object_ref) => py_object_ref.try_from_vm_value(vm).unwrap(),
                            Err(err) => {
                                let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                                panic!("{}", err_string);
                            }
                        }
                    });
                }
            }
            _ => panic!("{}", self.not_a_function_def_error()),
        }
    }
}
