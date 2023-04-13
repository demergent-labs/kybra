use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_type_ref_error(&self) -> Error {
        Error::NotATypeRef(self.create_error_message("This is is not a type ref", "", None))
    }
}
