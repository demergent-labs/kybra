use cdk_framework::{
    act::node::data_type::{func::Mode, Func, Primitive},
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::Message, generators::func, py_ast::kybra_types::KybraExpr, source_map::SourceMapped,
};

mod errors;

impl SourceMapped<&Located<StmtKind>> {
    pub fn to_func(&self) -> Result<Func, Message> {
        if !self.is_func() {
            return Err(self.todo_func_error());
        }
        match &self.node.node {
            StmtKind::AnnAssign { target, value, .. } => match &value {
                Some(value) => match &value.node {
                    ExprKind::Call { args, .. } => {
                        let name = match &target.node {
                            ExprKind::Name { id, .. } => id.clone(),
                            _ => return Err(self.todo_func_error()),
                        };
                        if args.len() != 1 {
                            return Err(self.todo_func_error());
                        }
                        let mode = match &args[0].node {
                            ExprKind::Subscript { value, .. } => match &value.node {
                                ExprKind::Name { id, .. } => {
                                    if !["Oneway", "Update", "Query"].contains(&id.as_str()) {
                                        return Err(self.todo_func_error());
                                    }
                                    id
                                }
                                _ => return Err(self.todo_func_error()),
                            },
                            _ => return Err(self.todo_func_error()),
                        }
                        .to_string();
                        let params = match &args[0].node {
                            ExprKind::Subscript { slice, .. } => match &slice.node {
                                ExprKind::Tuple { elts, .. } => {
                                    if elts.len() != 2 {
                                        return Err(self.todo_func_error());
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
                                        _ => return Err(self.todo_func_error()),
                                    }
                                }
                                _ => return Err(self.todo_func_error()),
                            },
                            _ => return Err(self.todo_func_error()),
                        };
                        let return_type = Box::from(if mode == "Oneway" {
                            Primitive::Void.to_data_type()
                            // TODO I am guessing void here but I am not sure
                        } else {
                            match &args[0].node {
                                ExprKind::Subscript { slice, .. } => match &slice.node {
                                    ExprKind::Tuple { elts, .. } => {
                                        if elts.len() != 2 {
                                            return Err(self.todo_func_error());
                                        }
                                        SourceMapped {
                                            node: &elts[1],
                                            source_map: self.source_map.clone(),
                                        }
                                        .to_data_type()
                                    }
                                    _ => return Err(self.todo_func_error()),
                                },
                                _ => return Err(self.todo_func_error()),
                            }
                        });
                        let mode = match mode.as_str() {
                            "Oneway" => Mode::Oneway,
                            "Update" => Mode::Update,
                            "Query" => Mode::Query,
                            _ => return Err(self.todo_func_error()),
                        };
                        Ok(Func {
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
                    _ => return Err(self.todo_func_error()),
                },
                None => return Err(self.todo_func_error()),
            },
            _ => return Err(self.todo_func_error()),
        }
    }

    pub fn as_func(&self) -> Option<Func> {
        self.to_func().ok()
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

    // TODO are we using this anywhere?
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
