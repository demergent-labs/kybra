use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_type_ref_error(&self) -> Vec<Message> {
        vec![self.create_error_message("This is is not a type ref", "", None)]
    }
}
