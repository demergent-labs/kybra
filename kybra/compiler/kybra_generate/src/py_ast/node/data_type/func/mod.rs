use cdk_framework::{
    act::node::{
        data_type::{func::Mode, Func, Primitive},
        DataType,
    },
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::Message, generators::func, py_ast::PyAst, source_map::SourceMapped};

mod errors;

impl PyAst {
    pub fn build_funcs(&self) -> Vec<Func> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_func())
            .collect()
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_func(&self) -> bool {
        match &self.node {
            ExprKind::Call { func, .. } => match &func.node {
                ExprKind::Name { id, .. } => id == "Func",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_func(&self, func_name: Option<String>) -> Result<Func, Message> {
        match &self.node {
            ExprKind::Call { args, .. } => {
                if args.len() != 1 {
                    return Err(self.todo_func_error());
                }
                let mode = self.get_func_mode()?;
                let params = self.get_func_params()?;
                let return_type = Box::from(self.get_func_return_type(mode.clone())?);
                let name = match func_name.clone() {
                    Some(name) => name,
                    None => return Err(self.inline_func_not_supported()),
                };
                Ok(Func {
                    to_vm_value: func::generate_func_to_vm_value(&name),
                    list_to_vm_value: func::generate_func_list_to_vm_value(&name),
                    from_vm_value: func::generate_func_from_vm_value(&name),
                    list_from_vm_value: func::generate_func_list_from_vm_value(&name),
                    name: func_name,
                    params,
                    return_type,
                    mode,
                })
            }
            _ => return Err(self.not_a_func_error()),
        }
    }

    fn get_func_mode(&self) -> Result<Mode, Message> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => Ok(match id.as_str() {
                        "Oneway" => Mode::Oneway,
                        "Update" => Mode::Update,
                        "Query" => Mode::Query,
                        _ => return Err(self.todo_func_error()),
                    }),
                    _ => Err(self.todo_func_error()),
                },
                _ => Err(self.todo_func_error()),
            },
            _ => Err(self.not_a_func_error()),
        }
    }

    fn get_func_params(&self) -> Result<Vec<DataType>, Message> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { slice, .. } => match &slice.node {
                    ExprKind::Tuple { elts, .. } => {
                        if elts.len() != 2 {
                            return Err(self.todo_func_error());
                        }
                        match &elts[0].node {
                            ExprKind::List { elts, .. } => Ok(elts
                                .iter()
                                .map(|elt| {
                                    SourceMapped {
                                        inner: elt,
                                        source_map: self.source_map.clone(),
                                    }
                                    .to_data_type()
                                })
                                .collect()),
                            _ => Err(self.todo_func_error()),
                        }
                    }
                    _ => Err(self.todo_func_error()),
                },
                _ => Err(self.todo_func_error()),
            },
            _ => Err(self.not_a_func_error()),
        }
    }

    fn get_func_return_type(&self, mode: Mode) -> Result<DataType, Message> {
        match &self.node {
            ExprKind::Call { args, .. } => match mode {
                Mode::Oneway => Ok(Primitive::Void.to_data_type()),
                _ => match &args[0].node {
                    ExprKind::Subscript { slice, .. } => match &slice.node {
                        ExprKind::Tuple { elts, .. } => {
                            if elts.len() != 2 {
                                return Err(self.todo_func_error());
                            }
                            Ok(SourceMapped {
                                inner: &elts[1],
                                source_map: self.source_map.clone(),
                            }
                            .to_data_type())
                        }
                        _ => return Err(self.todo_func_error()),
                    },
                    _ => return Err(self.todo_func_error()),
                },
            },
            _ => Err(self.not_a_func_error()),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn to_func(&self) -> Result<Func, Message> {
        if !self.is_func() {
            return Err(self.todo_func_error());
        }
        match &self.node {
            StmtKind::AnnAssign { target, value, .. } => match &value {
                Some(value) => {
                    let name = match &target.node {
                        ExprKind::Name { id, .. } => Some(id.clone()),
                        _ => None,
                    };
                    Ok(SourceMapped {
                        inner: value.as_ref(),
                        source_map: self.source_map.clone(),
                    }
                    .to_func(name)?)
                }
                None => return Err(self.todo_func_error()),
            },
            _ => return Err(self.todo_func_error()),
        }
    }

    pub fn as_func(&self) -> Option<Func> {
        self.to_func().ok()
    }

    pub fn is_func(&self) -> bool {
        match &self.node {
            StmtKind::AnnAssign {
                annotation, value, ..
            } => {
                let is_type_alias = match &annotation.node {
                    ExprKind::Name { id, .. } => id == "TypeAlias",
                    _ => false,
                };
                let is_func = match &value {
                    Some(value) => SourceMapped {
                        inner: value.as_ref(),
                        source_map: self.source_map.clone(),
                    }
                    .is_func(),
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

        match &self.node {
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
