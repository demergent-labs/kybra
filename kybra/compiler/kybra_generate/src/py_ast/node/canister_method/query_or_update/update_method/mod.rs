use cdk_framework::act::node::canister_method::{CanisterMethodType, UpdateMethod};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_update_methods(&self) -> KybraResult<Vec<UpdateMethod>> {
        let mut update_methods = vec![];
        let mut error_messages = vec![];

        self.get_stmt_kinds()
            .iter()
            .for_each(|stmt_kind| match stmt_kind.as_update_method() {
                Ok(Some(query_method)) => update_methods.push(query_method),
                Ok(None) => (),
                Err(errors) => error_messages.extend(errors),
            });

        if error_messages.is_empty() {
            Ok(update_methods)
        } else {
            Err(error_messages)
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_update_method(&self) -> KybraResult<Option<UpdateMethod>> {
        if !self.is_canister_method_type(CanisterMethodType::Update) {
            return Ok(None);
        }
        Ok(Some(UpdateMethod {
            definition: self.as_query_or_update_definition()?,
        }))
    }
}
