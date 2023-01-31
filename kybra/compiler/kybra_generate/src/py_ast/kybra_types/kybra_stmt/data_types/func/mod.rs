use cdk_framework::{
    act::node::{
        data_type::{func::Mode, Func},
        DataType,
    },
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    generators::func,
    py_ast::kybra_types::{KybraExpr, KybraStmt},
};

mod errors;

impl KybraStmt<'_> {
    pub fn as_func(&self) -> DataType {
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
                                                .to_data_type()
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
                            todo!();
                            // TODO I am guessing void here but I am not sure
                        } else {
                            match &args[0].node {
                                ExprKind::Subscript { slice, .. } => match &slice.node {
                                    ExprKind::Tuple { elts, .. } => {
                                        if elts.len() != 2 {
                                            todo!()
                                        }
                                        KybraExpr {
                                            located_expr: &elts[1],
                                            source_map: self.source_map,
                                        }
                                        .to_data_type()
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            }
                        });
                        let mode = match mode.as_str() {
                            "Oneway" => Mode::Oneway,
                            "Update" => Mode::Update,
                            "Query" => Mode::Query,
                            _ => todo!(),
                        };
                        DataType::Func(Func {
                            to_vm_value: func::generate_func_to_vm_value(&name),
                            list_to_vm_value: func::generate_func_list_to_vm_value(&name),
                            from_vm_value: func::generate_func_from_vm_value(&name),
                            list_from_vm_value: func::generate_func_list_from_vm_value(&name),
                            name: Some(name),
                            params,
                            return_type,
                            mode,
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

    pub fn get_func_args(&self) -> Option<&Vec<Located<ExprKind>>> {
        if !self.is_func() {
            return None;
        }

        match &self.stmt_kind.node {
            StmtKind::AnnAssign { value, .. } => match &value {
                Some(value) => match &value.node {
                    ExprKind::Call { args, .. } => Some(args),
                    _ => None,
                },
                None => None,
            },
            _ => None,
        }
    }
}
