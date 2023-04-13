use cdk_framework::act::node::{
    candid::{Func, Primitive},
    node_parts::mode::Mode,
    CandidType, ReturnType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

mod errors;
mod rust;

impl PyAst {
    pub fn build_funcs(&self) -> KybraResult<Vec<Func>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_func())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
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

    pub fn to_func(&self, name: Option<String>) -> KybraResult<Func> {
        match &self.node {
            ExprKind::Call { args, .. } => {
                if args.len() != 1 {
                    return Err(self.func_formatting_error());
                }
                let mode = self.get_func_mode()?;
                let params = self.build_func_params()?;
                let return_type = Box::from(ReturnType::new(
                    match self.build_func_return_type(mode.clone()) {
                        Ok(return_type) => return_type,
                        Err(err) => return Err(err),
                    },
                ));
                Ok(Func {
                    to_vm_value: |name: String| rust::generate_func_to_vm_value(&name),
                    list_to_vm_value: |name: String| rust::generate_func_list_to_vm_value(&name),
                    from_vm_value: |name: String| rust::generate_func_from_vm_value(&name),
                    list_from_vm_value: |name: String| {
                        rust::generate_func_list_from_vm_value(&name)
                    },
                    name,
                    params,
                    return_type,
                    mode,
                })
            }
            _ => return Err(crate::errors::unreachable()),
        }
    }

    fn get_func_mode(&self) -> KybraResult<Mode> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => Ok(match id.as_str() {
                        "Oneway" => Mode::Oneway,
                        "Update" => Mode::Update,
                        "Query" => Mode::Query,
                        _ => return Err(self.return_type_mode_error()),
                    }),
                    _ => Err(self.return_type_mode_error()),
                },
                _ => Err(self.return_type_mode_error()),
            },
            _ => Err(crate::errors::unreachable()),
        }
    }

    fn get_func_params(&self) -> KybraResult<Vec<SourceMapped<&Located<ExprKind>>>> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { slice, .. } => match &slice.node {
                    ExprKind::Tuple { elts, .. } => {
                        if elts.len() != 2 {
                            return Err(self.func_formatting_error());
                        }
                        match &elts[0].node {
                            ExprKind::List { elts, .. } => {
                                let thing = elts
                                    .iter()
                                    .map(|elt| SourceMapped::new(elt, self.source_map.clone()))
                                    .collect::<Vec<_>>();
                                Ok(thing)
                            }
                            _ => Err(self.func_formatting_error()),
                        }
                    }
                    _ => Err(self.func_formatting_error()),
                },
                _ => Err(self.func_formatting_error()),
            },
            _ => Err(crate::errors::unreachable()),
        }
    }

    fn get_func_return_type(&self) -> KybraResult<SourceMapped<&Located<ExprKind>>> {
        match &self.node {
            ExprKind::Call { args, .. } => match &args[0].node {
                ExprKind::Subscript { slice, .. } => match &slice.node {
                    ExprKind::Tuple { elts, .. } => {
                        if elts.len() != 2 {
                            return Err(self.func_formatting_error());
                        }
                        Ok(SourceMapped::new(&elts[1], self.source_map.clone()))
                    }
                    _ => return Err(self.func_formatting_error()),
                },
                _ => return Err(self.func_formatting_error()),
            },
            _ => Err(crate::errors::unreachable()),
        }
    }

    fn build_func_params(&self) -> KybraResult<Vec<CandidType>> {
        return crate::errors::collect_kybra_results(
            self.get_func_params()?
                .iter()
                .map(|param| param.to_candid_type())
                .collect(),
        );
    }

    fn build_func_return_type(&self, mode: Mode) -> KybraResult<CandidType> {
        match mode {
            Mode::Oneway => Ok(CandidType::Primitive(Primitive::Void)),
            _ => Ok(self.get_func_return_type()?.to_candid_type()?),
        }
    }

    pub fn func_uses_type_ref(&self, name: &str) -> bool {
        (match self.get_func_params() {
            Ok(params) => params.iter().any(|param| param.uses_type_ref(name)),
            Err(_) => false,
        } || match self.get_func_return_type() {
            Ok(return_type) => return_type.uses_type_ref(name),
            Err(_) => false,
        })
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_func(&self) -> KybraResult<Option<Func>> {
        if !self.is_func() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::Assign { targets, value, .. } => {
                if targets.len() != 1 {
                    return Err(self.multiple_func_targets_error());
                }

                let name = match &targets[0].node {
                    ExprKind::Name { id, .. } => Some(id.clone()),
                    _ => None,
                };

                Ok(Some(
                    SourceMapped::new(value.as_ref(), self.source_map.clone()).to_func(name)?,
                ))
            }
            _ => Err(crate::errors::unreachable()),
        }
    }

    pub fn is_func(&self) -> bool {
        match &self.node {
            StmtKind::Assign { value, .. } => {
                SourceMapped::new(value.as_ref(), self.source_map.clone()).is_func()
            }
            _ => false,
        }
    }

    pub fn func_uses_type_ref(&self, name: &str) -> bool {
        if let StmtKind::Assign { value, .. } = &self.node {
            return SourceMapped::new(value.as_ref(), self.source_map.clone())
                .func_uses_type_ref(name);
        }
        false
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
