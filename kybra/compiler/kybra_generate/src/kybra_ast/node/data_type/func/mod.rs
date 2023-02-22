use cdk_framework::{
    act::node::data_type::{func::Mode, Func},
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{generators::func, py_ast::kybra_types::KybraExpr, source_map::SourceMapped};

mod errors;

impl SourceMapped<'_, Located<StmtKind>> {
    // TODO make sure we are erroring instead of just noneing todo!()
    pub fn as_func(&self) -> Option<Func> {
        match &self.node.node {
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
                                                    source_map: self.source_map.clone(),
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
                                        SourceMapped {
                                            node: &elts[1],
                                            source_map: self.source_map.clone(),
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
                        Some(Func {
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
                    _ => None,
                },
                None => todo!(),
            },
            _ => None,
        }
    }

    pub fn is_func(&self) -> bool {
        match &self.node.node {
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

        match &self.node.node {
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
