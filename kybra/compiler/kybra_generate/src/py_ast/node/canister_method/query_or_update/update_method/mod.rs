use cdk_framework::act::node::canister_method::{CanisterMethodType, UpdateMethod};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    py_ast::PyAst,
    source_map::SourceMapped,
};

impl PyAst {
    pub fn build_update_methods(&self) -> Vec<UpdateMethod> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_update_method())
            .collect()
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_update_method(&self) -> Option<UpdateMethod> {
        if !self.is_canister_method_type(CanisterMethodType::Update) {
            return None;
        }
        Some(UpdateMethod {
            definition: self.as_query_or_update_definition()?,
        })
    }

    pub fn to_update_method(&self) -> Result<UpdateMethod, Message> {
        match self.as_update_method() {
            Some(update_method) => Ok(update_method),
            None => Err(self.create_error_message("title", "", None)),
        }
    }
}
