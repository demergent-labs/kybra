pub mod errors;
mod guard_function;

use cdk_framework::act::node;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    constants::GUARD_RESULT, errors::CollectResults, get_name::HasName, py_ast::PyAst,
    source_map::SourceMapped, Error,
};

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
        if let StmtKind::FunctionDef {
            name,
            returns: Some(returns),
            ..
        } = &self.node
        {
            if returns.get_name() == Some(GUARD_RESULT) {
                return Some(GuardFunction { name });
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
