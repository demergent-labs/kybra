pub mod errors;
mod guard_function;

use cdk_framework::act::node;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};

use self::errors::GuardFunctionParam;

struct GuardFunction<'a> {
    name: &'a String,
}

impl PyAst {
    pub fn build_guard_functions(&self) -> Result<Vec<node::GuardFunction>, Vec<Error>> {
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
    fn get_guard_function(&self) -> Option<GuardFunction> {
        if let StmtKind::FunctionDef { name, returns, .. } = &self.node {
            if let Some(returns) = returns {
                if let ExprKind::Name { id, .. } = &returns.node {
                    if id == "GuardResult" {
                        return Some(GuardFunction { name });
                    }
                }
            }
        }
        None
    }

    pub fn as_guard_function(&self) -> Result<Option<node::GuardFunction>, Vec<Error>> {
        match self.get_guard_function() {
            Some(guard_function) => {
                if self.has_params() {
                    return Err(GuardFunctionParam::err_from_stmt(self).into());
                }

                Ok(Some(node::GuardFunction {
                    body: guard_function::generate(guard_function.name),
                    name: guard_function.name.clone(),
                }))
            }
            None => Ok(None),
        }
    }

    fn has_params(&self) -> bool {
        match &self.node {
            StmtKind::FunctionDef { args, .. } => args.args.len() > 0,
            _ => false,
        }
    }
}
