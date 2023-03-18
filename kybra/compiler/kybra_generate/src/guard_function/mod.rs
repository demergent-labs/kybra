pub mod errors;
mod guard_function;

use cdk_framework::act::node::{CandidType, GuardFunction};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

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

    pub fn as_guard_function(&self) -> KybraResult<Option<GuardFunction>> {
        if !self.is_guard_function() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::FunctionDef { name, .. } => {
                if self.has_params() {
                    return Err(self.guard_functions_param_error());
                }
                if !self.returns_guard_result() {
                    return Err(self.guard_function_return_error());
                }

                Ok(Some(GuardFunction {
                    body: guard_function::generate(name),
                    name: name.clone(),
                }))
            }
            _ => return Err(crate::errors::unreachable()),
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

impl PyAst {
    pub fn build_guard_functions(&self) -> KybraResult<Vec<GuardFunction>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_guard_function())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }
}
