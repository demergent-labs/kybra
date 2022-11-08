use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::KybraExpr;
use cdk_framework::{
    nodes::{data_type_nodes::ActPrimitiveLit, ActFnParam},
    ActDataType, CanisterMethod, CanisterMethodType, ToActDataType,
};

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn is_canister_method_stmt(&self) -> bool {
        self.is_canister_method_type(CanisterMethodType::Update)
            || self.is_canister_method_type(CanisterMethodType::Query)
    }

    pub fn is_canister_method_type(&self, canister_method_type: CanisterMethodType) -> bool {
        self.is_decorator_same_as(match canister_method_type {
            CanisterMethodType::Heartbeat => "heartbeat",
            CanisterMethodType::Init => "init",
            CanisterMethodType::InspectMessage => "inspect_message",
            CanisterMethodType::PostUpgrade => "post_upgrade",
            CanisterMethodType::PreUpgrade => "pre_upgrade",
            CanisterMethodType::Query => "query",
            CanisterMethodType::Update => "update",
        })
    }

    fn is_decorator_same_as(&self, decorator_name: &str) -> bool {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { decorator_list, .. } => {
                decorator_list
                    .iter()
                    .any(|expr_kind| match &expr_kind.node {
                        ExprKind::Name { id, .. } => id == decorator_name,
                        _ => false,
                    })
            }
            _ => false,
        }
    }

    fn build_act_params(&self) -> Vec<ActFnParam> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { args, .. } => {
                args.args
                    .iter()
                    .fold(vec![], |acc, arg| match &arg.node.annotation {
                        Some(annotation) => {
                            let name = arg.node.arg.clone();
                            let kybra_annotation = KybraExpr {
                                located_expr: &annotation,
                                source_map: &self.source_map,
                            };
                            let data_type = kybra_annotation.to_act_data_type(&None);
                            vec![acc, vec![ActFnParam { name, data_type }]].concat()
                        }
                        None => todo!("Param type needs type annotation"),
                    })
            }
            _ => todo!(),
        }
    }

    fn is_manual(&self) -> bool {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { returns, .. } => match returns {
                Some(returns) => match &returns.node {
                    ExprKind::Subscript { value, .. } => match &value.node {
                        ExprKind::Name { id, .. } => id == "manual",
                        _ => false,
                    },
                    _ => false,
                },
                None => false,
            },
            _ => false,
        }
    }

    pub fn as_canister_method(&self) -> Option<CanisterMethod> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { name, .. } => {
                let body = self.generate_body();
                let params = self.build_act_params();
                let return_type = self.build_act_return_type();

                Some(CanisterMethod {
                    body,
                    params,
                    is_manual: self.is_manual(),
                    name: name.clone(),
                    return_type,
                })
            }
            _ => None,
        }
    }

    fn build_act_return_type(&self) -> ActDataType {
        let returns = match &self.stmt_kind.node {
            StmtKind::FunctionDef { returns, .. } => returns,
            _ => panic!("Unreachable"),
        };

        match returns {
            Some(return_type) => {
                let kybra_return_type = KybraExpr {
                    located_expr: &return_type,
                    source_map: &self.source_map,
                };
                kybra_return_type.to_act_data_type(&None)
            }
            None => ActPrimitiveLit::Void.to_act_data_type(&None),
        }
    }

    fn generate_body(&self) -> TokenStream {
        let args = match &self.stmt_kind.node {
            StmtKind::FunctionDef { args, .. } => args,
            _ => panic!("Unreachable"),
        };

        let name = match self.get_name() {
            Some(name) => name,
            None => todo!(),
        };

        let param_conversions = args.args.iter().map(|arg| {
            let param_name = format_ident!("{}", arg.node.arg);
            quote! {
                #param_name.try_into_vm_value(vm).unwrap()
            }
        });

        let params_comma = if param_conversions.len() == 1 {
            quote!(,)
        } else {
            quote!()
        };

        let return_expression = self.generate_return_expression();

        quote! {
            unsafe {
                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                let vm = &_kybra_interpreter.vm;

                let method_py_object_ref = _kybra_scope.globals.get_item(#name, vm).unwrap();

                let invoke_result = vm.invoke(&method_py_object_ref, (#(#param_conversions),*#params_comma));

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
}

fn type_is_null_or_void(act_type: ActDataType) -> bool {
    match act_type {
        ActDataType::Primitive(primitive) => match primitive.act_type {
            cdk_framework::nodes::data_type_nodes::LiteralOrTypeAlias::Literal(literal) => {
                match literal {
                    ActPrimitiveLit::Null => true,
                    ActPrimitiveLit::Void => true,
                    _ => false,
                }
            }
            cdk_framework::nodes::data_type_nodes::LiteralOrTypeAlias::TypeAlias(type_alias) => {
                match type_alias.aliased_type {
                    ActPrimitiveLit::Null => true,
                    ActPrimitiveLit::Void => true,
                    _ => false,
                }
            }
        },
        _ => false,
    }
}
