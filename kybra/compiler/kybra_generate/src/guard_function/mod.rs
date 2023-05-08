pub mod errors;
mod guard_function;

use cdk_framework::{
    act::node::{CandidType, GuardFunction},
    traits::CollectResults,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CollectResults as OtherCollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

use self::errors::{GuardFunctionParam, GuardFunctionReturn};

impl PyAst {
    pub fn build_guard_functions(&self) -> Result<Vec<GuardFunction>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_guard_function())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_guard_function(&self) -> bool {
        if let StmtKind::FunctionDef { .. } = self.node {
            match self.build_return_type() {
                Ok(return_type) => {
                    if let CandidType::TypeRef(type_ref) = return_type {
                        return type_ref.name == "GuardResult";
                    }
                    false
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn as_guard_function(&self) -> Result<Option<GuardFunction>, Vec<Error>> {
        if !self.is_guard_function() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::FunctionDef { name, .. } => {
                (
                    match !self.has_params() {
                        true => Ok(()),
                        false => Err(GuardFunctionParam::err_from_stmt(self).into()),
                    },
                    match self.returns_guard_result() {
                        true => Ok(()),
                        false => Err(GuardFunctionReturn::err_from_stmt(self).into()),
                    },
                )
                    .collect_results()?;

                Ok(Some(GuardFunction {
                    body: guard_function::generate(name),
                    name: name.clone(),
                }))
            }
            _ => return Err(Unreachable::new_err().into()),
        }
    }

    fn has_params(&self) -> bool {
        match &self.node {
            StmtKind::FunctionDef { args, .. } => args.args.len() > 0,
            _ => false,
        }
    }

    fn returns_guard_result(&self) -> bool {
        match &self.node {
            StmtKind::FunctionDef { returns, .. } => {
                if let Some(returns) = returns {
                    match &returns.node {
                        ExprKind::Name { id, .. } => id == "GuardResult",
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
