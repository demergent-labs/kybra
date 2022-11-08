use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::{KybraExpr, KybraStmt};
use cdk_framework::{
    nodes::data_type_nodes::{
        act_funcs::{Func, FuncTypeAlias},
        ActFunc, LiteralOrTypeAlias,
    },
    ActDataType, ToActDataType,
};

mod errors;

impl KybraStmt<'_> {
    pub fn as_func(&self) -> ActDataType {
        match &self.stmt_kind.node {
            StmtKind::AnnAssign { target, value, .. } => match &value {
                Some(value) => match &value.node {
                    ExprKind::Call { args, .. } => {
                        let name = match &target.node {
                            ExprKind::Name { id, .. } => id.clone(),
                            _ => todo!(),
                        };
                        if args.len() != 1 {
                            todo!()
                        }
                        let mode = match &args[0].node {
                            ExprKind::Subscript { value, .. } => match &value.node {
                                ExprKind::Name { id, .. } => {
                                    if !["Oneway", "Update", "Query"].contains(&id.as_str()) {
                                        todo!()
                                    }
                                    id
                                }
                                _ => todo!(),
                            },
                            _ => todo!(),
                        }
                        .to_string();
                        let params = match &args[0].node {
                            ExprKind::Subscript { slice, .. } => match &slice.node {
                                ExprKind::Tuple { elts, .. } => {
                                    if elts.len() != 2 {
                                        todo!()
                                    }
                                    match &elts[0].node {
                                        ExprKind::List { elts, .. } => elts
                                            .iter()
                                            .map(|elt| {
                                                KybraExpr {
                                                    located_expr: elt,
                                                    source_map: self.source_map,
                                                }
                                                .to_act_data_type(&None)
                                            })
                                            .collect(),
                                        _ => todo!(),
                                    }
                                }
                                _ => todo!(),
                            },
                            _ => todo!(),
                        };
                        let return_type = Box::from(if mode == "Oneway" {
                            None
                        } else {
                            match &args[0].node {
                                ExprKind::Subscript { slice, .. } => match &slice.node {
                                    ExprKind::Tuple { elts, .. } => {
                                        if elts.len() != 2 {
                                            todo!()
                                        }
                                        Some(
                                            KybraExpr {
                                                located_expr: &elts[1],
                                                source_map: self.source_map,
                                            }
                                            .to_act_data_type(&None),
                                        )
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            }
                        });
                        ActDataType::Func(ActFunc {
                            act_type: LiteralOrTypeAlias::TypeAlias(FuncTypeAlias {
                                func: Func {
                                    name,
                                    params,
                                    return_type,
                                    mode,
                                },
                            }),
                        })
                    }
                    _ => todo!(),
                },
                None => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn is_func(&self) -> bool {
        match &self.stmt_kind.node {
            StmtKind::AnnAssign {
                annotation, value, ..
            } => {
                let is_type_alias = match &annotation.node {
                    ExprKind::Name { id, .. } => id == "TypeAlias",
                    _ => false,
                };
                let is_func = match &value {
                    Some(value) => match &value.node {
                        ExprKind::Call { func, .. } => match &func.node {
                            ExprKind::Name { id, .. } => id == "Func",
                            _ => false,
                        },
                        _ => false,
                    },
                    None => false,
                };
                is_type_alias && is_func
            }
            _ => false,
        }
    }
}
