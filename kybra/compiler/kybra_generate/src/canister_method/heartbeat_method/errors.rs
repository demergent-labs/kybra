use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_heartbeat_allowed_error(&self) -> Message {
        self.create_error_message(
            "Only one heartbeat function allowed",
            "Duplicate heartbeat here",
            None,
        )
    }

    pub fn heartbeat_method_must_return_void_error(&self) -> Vec<Message> {
        vec![self.create_error_message(
            "Heartbeat method must have an explicit void return type annotation",
            "",
            None,
        )]
    }
}
