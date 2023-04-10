use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_opt_error(&self) -> Vec<Message> {
        vec![self.create_error_message("This is is not an Opt", "", None)]
    }
}
