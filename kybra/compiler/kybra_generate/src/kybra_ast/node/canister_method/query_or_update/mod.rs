use cdk_framework::{
    act::node::{data_type::Primitive, param::Param, DataType},
    ToDataType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{generators::tuple, source_map::SourceMapped};

pub mod query_method;
pub mod update_method;

impl SourceMapped<&Located<StmtKind>> {
    pub fn generate_body(&self) -> TokenStream {
        let act_params = self.build_act_params();

        let name = match self.get_name() {
            Some(name) => name,
            None => todo!(),
        };

        let param_conversions = act_params
            .iter()
            .map(|param| {
                let name = format_ident!("{}", param.prefixed_name());
                quote! {
                    #name.try_into_vm_value(vm).unwrap()
                }
            })
            .collect();

        let params = tuple::generate_tuple(&param_conversions);

        let return_expression = self.generate_return_expression();

        quote! {
            unsafe {
                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                let vm = &_kybra_interpreter.vm;

                let method_py_object_ref = _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item(#name, vm), vm);

                let invoke_result = vm.invoke(&method_py_object_ref, #params);

                match invoke_result {
                    Ok(py_object_ref) => {
                        let _kybra_final_return_value = _kybra_async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;

                        #return_expression
                    },
                    Err(err) => {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                        panic!("{}", err_string);
                    }
                }
            }
        }
    }

    pub fn is_manual(&self) -> bool {
        match &self.node.node {
            StmtKind::FunctionDef { returns, .. } => match returns {
                Some(returns) => SourceMapped {
                    node: returns.as_ref(),
                    source_map: self.source_map.clone(),
                }
                .is_manual(),
                None => false,
            },
            _ => false,
        }
    }

    pub fn is_async(&self) -> bool {
        let returns = match &self.node.node {
            StmtKind::FunctionDef { returns, .. } => returns,
            _ => return false,
        };

        match returns {
            Some(returns) => match &returns.node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => id == "Async",
                    _ => false,
                },
                _ => false,
            },
            None => false,
        }
    }

    fn generate_return_expression(&self) -> TokenStream {
        if self.is_manual() {
            return quote! {
                ic_cdk::api::call::ManualReply::empty()
            };
        }

        let return_type = self.build_act_return_type();
        if type_is_null_or_void(return_type) {
            return quote! {
                return;
            };
        }

        quote! {
            _kybra_final_return_value.try_from_vm_value(vm).unwrap()
        }
    }

    pub fn build_act_return_type(&self) -> DataType {
        let returns = match &self.node.node {
            StmtKind::FunctionDef { returns, .. } => returns,
            _ => panic!("Unreachable"),
        };

        match returns {
            Some(return_type) => SourceMapped {
                node: &**return_type,
                source_map: self.source_map.clone(),
            }
            .to_data_type(),
            None => Primitive::Void.to_data_type(),
        }
    }

    pub fn build_act_params(&self) -> Vec<Param> {
        match &self.node.node {
            StmtKind::FunctionDef { args, .. } => {
                args.args
                    .iter()
                    .fold(vec![], |acc, arg| match &arg.node.annotation {
                        Some(annotation) => {
                            let name = arg.node.arg.clone();
                            let kybra_annotation = SourceMapped {
                                node: &**annotation,
                                source_map: self.source_map.clone(),
                            };
                            let data_type = kybra_annotation.to_data_type();
                            vec![
                                acc,
                                vec![Param {
                                    name,
                                    type_: data_type,
                                }],
                            ]
                            .concat()
                        }
                        None => todo!("Param type needs type annotation"),
                    })
            }
            _ => todo!(),
        }
    }

    pub fn get_guard_function_name(&self) -> Option<String> {
        match &self.node.node {
            StmtKind::FunctionDef { decorator_list, .. } => {
                decorator_list
                    .iter()
                    .fold(None, |_, decorator| match &decorator.node {
                        ExprKind::Call { keywords, .. } => {
                            keywords.iter().fold(None, |_, keyword| {
                                if let Some(arg) = &keyword.node.arg {
                                    if arg == "guard" {
                                        match &keyword.node.value.node {
                                            ExprKind::Constant { value, .. } => match value {
                                                Constant::Str(string) => Some(string.to_string()),
                                                _ => None,
                                            },
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                        }
                        _ => None,
                    })
            }
            _ => None,
        }
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_manual(&self) -> bool {
        match &self.node.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => {
                    if id == "manual" {
                        return true;
                    } else {
                        return SourceMapped {
                            node: slice.as_ref(),
                            source_map: self.source_map.clone(),
                        }
                        .is_manual();
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }
}

fn type_is_null_or_void(act_type: DataType) -> bool {
    match act_type {
        DataType::Primitive(primitive) => match primitive {
            Primitive::Null => true,
            Primitive::Void => true,
            _ => false,
        },
        _ => false,
    }
}
