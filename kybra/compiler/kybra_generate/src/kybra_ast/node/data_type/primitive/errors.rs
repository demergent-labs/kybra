use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<'_, Located<ExprKind>> {
    pub fn not_a_primitive_error(&self) -> Message {
        self.create_error_message("Not a primitive", "TODO", None)
    }
}
