use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn must_be_subscript_error(&self) -> Vec<Message> {
        vec![self.create_error_message("", "", None)]
    }
}
