use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_a_primitive_error(&self) -> Error {
        Error::NotAPrimitive(self.create_error_message("Not a primitive", "TODO", None))
    }
}
