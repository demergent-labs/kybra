use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_post_upgrade_method_allowed_error(&self) -> Message {
        self.create_error_message(
            "Only one inspect message allowed",
            "Duplicate inspect here",
            None,
        )
    }

    pub fn post_upgrade_method_must_return_void_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Post upgrade method must return void explicitly", "", None)]
    }
}
