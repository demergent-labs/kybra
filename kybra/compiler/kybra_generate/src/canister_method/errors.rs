use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn guard_function_name_error(&self) -> Vec<Message> {
        vec![self.create_error_message(
            "Guard function name must be a string or function reference",
            "",
            None,
        )]
    }
}
