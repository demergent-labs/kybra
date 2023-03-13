use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::act::node::canister_method::{CanisterMethodType, QueryMethod};
use rustpython_parser::ast::{Located, StmtKind};

impl PyAst {
    pub fn build_query_methods(&self) -> KybraResult<Vec<QueryMethod>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_query_method())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_query_method(&self) -> KybraResult<Option<QueryMethod>> {
        if !self.is_canister_method_type(CanisterMethodType::Query) {
            return Ok(None);
        }
        Ok(Some(QueryMethod {
            definition: self.as_query_or_update_definition()?,
        }))
    }
}
