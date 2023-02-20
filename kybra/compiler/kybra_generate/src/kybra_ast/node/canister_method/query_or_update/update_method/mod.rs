use cdk_framework::act::node::canister_method::{CanisterMethodType, UpdateMethod};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<'_, Located<StmtKind>> {
    pub fn as_update_method(&self) -> Option<UpdateMethod> {
        if !self.is_canister_method_type(CanisterMethodType::Update) {
            return None;
        }
        match &self.node.node {
            StmtKind::FunctionDef { name, .. } => Some(UpdateMethod {
                body: self.generate_body(),
                params: self.build_act_params(),
                is_manual: self.is_manual(),
                name: name.clone(),
                return_type: self.build_act_return_type(),
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
