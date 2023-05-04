use cdk_framework::{
    act::node::{
        candid::{Func, Primitive},
        node_parts::mode::Mode,
        CandidType, ReturnType,
    },
    traits::CollectResults,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CollectResults as CrateCollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

use self::errors::{FuncFormatting, ReturnTypeMode};

use super::errors::NotExactlyOneTarget;

pub mod errors;
mod rust;

impl PyAst {
    pub fn build_funcs(&self) -> Result<Vec<Func>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_func())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<ExprKind>> {
    fn is_func(&self) -> bool {
        match &self.node {
            ExprKind::Call { func, .. } => match &func.node {
                ExprKind::Name { id, .. } => id == "Func",
                _ => false,
            },
            _ => false,
        }
    }

    fn to_func(&self, name: Option<String>) -> Result<Func, Vec<Error>> {
        match &self.node {
            ExprKind::Call { args, .. } => {
                if args.len() != 1 {
                    return Err(FuncFormatting::err_from_expr(self).into());
                }
                let (mode,) = (self.get_func_mode().map_err(Error::into),).collect_results()?;
                let (params, return_type) = (
                    self.build_func_params(),
                    self.build_func_return_type(mode.clone()),
                )
                    .collect_results()?;
                Ok(Func {
                    to_vm_value: |name: String| rust::generate_func_to_vm_value(&name),
                    list_to_vm_value: |name: String| rust::generate_func_list_to_vm_value(&name),
                    from_vm_value: |name: String| rust::generate_func_from_vm_value(&name),
                    list_from_vm_value: |name: String| {
                        rust::generate_func_list_from_vm_value(&name)
                    },
                    name,
                    params,
                    return_type: Box::from(ReturnType::new(return_type)),
                    mode,
                })
            }
            _ => return Err(Unreachable::new_err().into()),
        }
    }

    fn get_func_mode(&self) -> Result<Mode, Error> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => Ok(match id.as_str() {
                        "Oneway" => Mode::Oneway,
                        "Update" => Mode::Update,
                        "Query" => Mode::Query,
                        _ => return Err(ReturnTypeMode::err_from_expr(self)),
                    }),
                    _ => Err(ReturnTypeMode::err_from_expr(self)),
                },
                _ => Err(ReturnTypeMode::err_from_expr(self)),
            },
            _ => Err(Unreachable::new_err()),
        }
    }

    fn get_func_params(&self) -> Result<Vec<SourceMapped<&Located<ExprKind>>>, Error> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { slice, .. } => match &slice.node {
                    ExprKind::Tuple { elts, .. } => {
                        if elts.len() != 2 {
                            return Err(FuncFormatting::err_from_expr(self));
                        }
                        match &elts[0].node {
                            ExprKind::List { elts, .. } => Ok(elts
                                .iter()
                                .map(|elt| SourceMapped::new(elt, self.source_map.clone()))
                                .collect::<Vec<_>>()),
                            _ => Err(FuncFormatting::err_from_expr(self)),
                        }
                    }
                    _ => Err(FuncFormatting::err_from_expr(self)),
                },
                _ => Err(FuncFormatting::err_from_expr(self)),
            },
            _ => Err(Unreachable::new_err()),
        }
    }

    fn get_func_return_type(&self) -> Result<SourceMapped<&Located<ExprKind>>, Error> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { slice, .. } => match &slice.node {
                    ExprKind::Tuple { elts, .. } => {
                        if elts.len() != 2 {
                            return Err(FuncFormatting::err_from_expr(self));
                        }
                        Ok(SourceMapped::new(&elts[1], self.source_map.clone()))
                    }
                    _ => return Err(FuncFormatting::err_from_expr(self)),
                },
                _ => return Err(FuncFormatting::err_from_expr(self)),
            },
            _ => Err(Unreachable::new_err()),
        }
    }

    fn build_func_params(&self) -> Result<Vec<CandidType>, Vec<Error>> {
        Ok(match self.get_func_params() {
            Ok(func_params) => func_params,
            Err(err) => return Err(err.into()),
        }
        .iter()
        .map(|param| param.to_candid_type())
        .collect_results()?)
    }

    fn build_func_return_type(&self, mode: Mode) -> Result<CandidType, Vec<Error>> {
        match mode {
            Mode::Oneway => Ok(CandidType::Primitive(Primitive::Void)),
            _ => Ok(match self.get_func_return_type() {
                Ok(return_type) => return_type,
                Err(err) => return Err(err.into()),
            }
            .to_candid_type()?),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_func(&self) -> Result<Option<Func>, Vec<Error>> {
        if !self.is_func() {
            return Ok(None);
        }
        let name = match &self.node {
            StmtKind::Assign { targets, .. } => {
                if targets.len() != 1 {
                    return Err(NotExactlyOneTarget::err_from_stmt(self).into());
                }

                match &targets[0].node {
                    ExprKind::Name { id, .. } => Some(id.clone()),
                    _ => None,
                }
            }
            StmtKind::AnnAssign { target, .. } => match &target.node {
                ExprKind::Name { id, .. } => Some(id.clone()),
                _ => None,
            },
            _ => None,
        };
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => Ok(Some(
                SourceMapped::new(value.as_ref(), self.source_map.clone()).to_func(name)?,
            )),
            _ => Err(Unreachable::new_err().into()),
        }
    }

    pub fn is_func(&self) -> bool {
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => SourceMapped::new(value.as_ref(), self.source_map.clone()).is_func(),
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
