use crate::{py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::act::node::canister_method::{CanisterMethodType, QueryMethod};
use rustpython_parser::ast::{Located, StmtKind};

impl PyAst {
    pub fn build_query_methods(&self) -> Vec<QueryMethod> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_query_method())
            .collect()
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_query_method(&self) -> Option<QueryMethod> {
        if !self.is_canister_method_type(CanisterMethodType::Query) {
            return None;
        }
        Some(QueryMethod {
            definition: self.as_query_or_update_definition()?,
        })
    }
}
