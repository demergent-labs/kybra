use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<'_, Located<StmtKind>> {
    pub fn multiple_targets_error(&self) -> Message {
        Message::Error(self.create_message("", "", None))
    }

    pub fn invalid_target_error(&self) -> Message {
        Message::Error(self.create_message("", "", None))
    }

    pub fn not_a_tuple_error(&self) -> Message {
        Message::Error(self.create_message("", "", None))
    }
}
