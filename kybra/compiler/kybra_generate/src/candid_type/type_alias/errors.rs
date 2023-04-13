use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn must_be_subscript_error(&self) -> Error {
        Error::MustBeSubscript(self.create_error_message("", "", None))
    }
}
