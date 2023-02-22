use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{generators::tuple, py_ast::kybra_types::KybraStmt, source_map::SourceMapped};

pub mod heartbeat;
pub mod init;
pub mod inspect_message;
pub mod post_upgrade;
pub mod pre_upgrade;
pub mod query_and_update;

pub fn generate_call_to_py_function(statement: &KybraStmt) -> proc_macro2::TokenStream {
    match statement.stmt_kind.node {
        rustpython_parser::ast::StmtKind::FunctionDef { .. } => {
            let function_name = statement.get_function_name();
            let act_params = statement.build_params();

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
        _ => panic!("{}", statement.not_a_function_def_error()),
    }
}

pub fn generate_call_to_py_function_cdk_refactor_name(
    statement: &SourceMapped<&Located<StmtKind>>,
) -> TokenStream {
    match statement.node.node {
        rustpython_parser::ast::StmtKind::FunctionDef { .. } => {
            let function_name = statement.get_function_name();
            let act_params = statement.build_params();

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
        _ => panic!("{}", statement.not_a_function_def_error()),
    }
}
