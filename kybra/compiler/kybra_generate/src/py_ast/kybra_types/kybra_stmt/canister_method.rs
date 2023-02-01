use cdk_framework::{
    act::node::{
        canister_method::{ActFnParam, QueryMethod, UpdateMethod},
        data_type::primitive::ActPrimitiveLit,
        DataType,
    },
    CanisterMethodType, ToActDataType,
};
use rustpython_parser::ast::{ExprKind, StmtKind};

use super::KybraStmt;
use crate::{generators::canister_methods::query_and_update, py_ast::kybra_types::KybraExpr};

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

    pub fn build_act_params(&self) -> Vec<ActFnParam> {
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

    pub fn is_manual(&self) -> bool {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { returns, .. } => match returns {
                Some(returns) => KybraExpr {
                    located_expr: returns,
                    source_map: self.source_map,
                }
                .is_manual(),
                None => false,
            },
            _ => false,
        }
    }

    pub fn is_async(&self) -> bool {
        let returns = match &self.stmt_kind.node {
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

    pub fn as_update_method(&self) -> Option<UpdateMethod> {
        if !self.is_canister_method_type(CanisterMethodType::Update) {
            return None;
        }
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { name, .. } => {
                let body = query_and_update::generate_body(&self);
                let params = self.build_act_params();
                let return_type = self.build_act_return_type();

                Some(UpdateMethod {
                    body,
                    params,
                    is_manual: self.is_manual(),
                    name: name.clone(),
                    return_type,
                    is_async: self.is_async(),
                    cdk_name: "kybra".to_string(),
                    function_guard_name: None,
                })
            }
            _ => None,
        }
    }

    pub fn as_query_method(&self) -> Option<QueryMethod> {
        if !self.is_canister_method_type(CanisterMethodType::Query) {
            return None;
        }
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { name, .. } => {
                let body = query_and_update::generate_body(&self);
                let params = self.build_act_params();
                let return_type = self.build_act_return_type();

                Some(QueryMethod {
                    body,
                    params,
                    is_manual: self.is_manual(),
                    name: name.clone(),
                    return_type,
                    is_async: self.is_async(),
                    cdk_name: "kybra".to_string(),
                    function_guard_name: None,
                })
            }
            _ => None,
        }
    }

    pub fn build_act_return_type(&self) -> DataType {
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
}
