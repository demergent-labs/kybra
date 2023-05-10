use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};
use cdk_framework::act::node::canister_method::{CanisterMethodType, QueryMethod};
use rustpython_parser::ast::{Located, StmtKind};

impl PyAst {
    pub fn build_query_methods(&self) -> Result<Vec<QueryMethod>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_query_method())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_query_method(&self) -> Result<Option<QueryMethod>, Vec<Error>> {
        if !self.is_canister_method_type(CanisterMethodType::Query) {
            return Ok(None);
        }
        Ok(Some(QueryMethod {
            definition: self.as_query_or_update_definition()?,
        }))
    }
}
