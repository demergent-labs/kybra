pub mod errors;

use std::{collections::HashSet, ops::Deref};

use cdk_framework::act::node::GuardFunction;
use rustpython_parser::ast::{Located, Mod, StmtKind};

use crate::{
    errors::KybraResult, generators::guard_function, py_ast::PyAst, source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_guard_function(&self, guard_function_names: &Vec<String>) -> bool {
        match &self.node {
            StmtKind::FunctionDef { name, .. } => guard_function_names.contains(name),
            _ => false,
        }
    }

    pub fn as_guard_function(
        &self,
        guard_function_names: &Vec<String>,
    ) -> KybraResult<Option<GuardFunction>> {
        if !self.is_guard_function(guard_function_names) {
            return Ok(None);
        }
        match &self.node {
            StmtKind::FunctionDef { name, .. } => {
                if self.has_params() {
                    return Err(self.guard_functions_param_error());
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
}

impl SourceMapped<Mod> {
    fn get_guard_function_names(&self) -> Vec<String> {
        let guard_function_names: HashSet<_> = match &self.deref() {
            Mod::Module { body, .. } => body
                .iter()
                .filter_map(|stmt_kind| {
                    SourceMapped::new(stmt_kind, self.source_map.clone()).get_guard_function_name()
                })
                .collect(),
            _ => HashSet::new(),
        };
        guard_function_names.iter().cloned().collect()
    }
}

impl PyAst {
    pub fn build_guard_functions(&self) -> KybraResult<Vec<GuardFunction>> {
        let guard_function_names = self.get_guard_function_names();
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| {
                    source_mapped_stmt_kind.as_guard_function(&guard_function_names)
                })
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }

    pub fn get_guard_function_names(&self) -> Vec<String> {
        let guard_function_names =
            self.source_mapped_mods
                .iter()
                .fold(HashSet::new(), |mut acc, kybra_program| {
                    acc.extend(kybra_program.get_guard_function_names());
                    acc
                });
        guard_function_names.iter().cloned().collect()
    }
}
