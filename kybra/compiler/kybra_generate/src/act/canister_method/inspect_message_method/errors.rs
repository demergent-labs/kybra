use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_inspect_message_method_allowed_error(&self) -> Message {
        self.create_error_message(
            "Only one inspect message allowed",
            "Duplicate inspect here",
            None,
        )
    }
}
