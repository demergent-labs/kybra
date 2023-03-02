use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::act::node::canister_method::{CanisterMethodType, QueryMethod};
use rustpython_parser::ast::{Located, StmtKind};

impl PyAst {
    pub fn build_query_methods(&self) -> KybraResult<Vec<QueryMethod>> {
        let (query_methods, error_messages): (Vec<_>, Vec<_>) = self.get_stmt_kinds().iter().fold(
            (vec![], vec![]),
            |(mut query_methods, mut error_messages), stmt_kind| match stmt_kind.as_query_method() {
                Ok(Some(query_method)) => {
                    query_methods.push(query_method);
                    (query_methods, error_messages)
                }
                Ok(None) => (query_methods, error_messages),
                Err(messages) => {
                    error_messages.extend(messages);
                    (query_methods, error_messages)
                }
            },
        );

        if error_messages.is_empty() {
            Ok(query_methods)
        } else {
            Err(error_messages)
        }
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
