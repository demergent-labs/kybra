use cdk_framework::act::node::canister_method::{CanisterMethodType, UpdateMethod};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    generators::canister_methods::query_and_update,
    kybra_ast::NewPyAst,
    source_map::SourceMapped,
};

impl NewPyAst {
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
        match &self.node.node {
            StmtKind::FunctionDef { name, .. } => Some(UpdateMethod {
                body: query_and_update::generate_body(self),
                params: self.build_params(),
                is_manual: self.is_manual(),
                name: name.clone(),
                return_type: self.build_return_type(),
                is_async: self.is_async(),
                cdk_name: "kybra".to_string(),
                guard_function_name: self.get_guard_function_name(),
            }),
            _ => None,
        }
    }

    pub fn to_update_method(&self) -> Result<UpdateMethod, Message> {
        match self.as_update_method() {
            Some(update_method) => Ok(update_method),
            None => Err(self.create_error_message("title", "", None)),
        }
    }
}
