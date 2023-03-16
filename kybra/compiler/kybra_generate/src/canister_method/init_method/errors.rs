use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_init_allowed_error(&self) -> Message {
        self.create_error_message(
            "Only one init function allowed",
            "Duplicate init here",
            None,
        )
    }

    pub fn init_method_must_return_void_error(&self) -> Vec<Message> {
        vec![self.create_error_message(
            "Init method must have an explicit void return type annotation",
            "",
            None,
        )]
    }
}
