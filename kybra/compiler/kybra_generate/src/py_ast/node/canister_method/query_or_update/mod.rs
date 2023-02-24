use cdk_framework::{
    act::node::{data_type::Primitive, DataType},
    ToDataType,
};
use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::source_map::SourceMapped;

pub mod query_method;
pub mod update_method;

impl SourceMapped<&Located<StmtKind>> {
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

    pub fn build_return_type(&self) -> DataType {
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