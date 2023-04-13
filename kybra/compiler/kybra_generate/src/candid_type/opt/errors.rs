use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_opt_error(&self) -> Error {
        Error::NotAnOpt(self.create_error_message("This is is not an Opt", "", None))
    }
}
