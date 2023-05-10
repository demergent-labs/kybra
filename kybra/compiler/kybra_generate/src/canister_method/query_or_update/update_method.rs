use cdk_framework::act::node::canister_method::{CanisterMethodType, UpdateMethod};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::CollectResults, kybra_unreachable, py_ast::PyAst, source_map::SourceMapped, Error,
};

impl PyAst {
    pub fn build_update_methods(&self) -> Result<Vec<UpdateMethod>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_update_method())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_update_method(&self) -> Result<Option<UpdateMethod>, Vec<Error>> {
        if !self.is_canister_method_type(CanisterMethodType::Update) {
            return Ok(None);
        }
        let definition = match self.as_query_or_update_definition()? {
            Some(def) => def,
            None => kybra_unreachable!(),
        };
        Ok(Some(UpdateMethod { definition }))
    }
}
