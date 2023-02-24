use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
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

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_tuple_error(&self) -> Message {
        self.create_error_message("This is is not a tuple", "", None)
    }
}
