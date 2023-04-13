use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub(super) fn record_target_must_be_a_name_error(&self) -> Error {
        Error::TargetMustBeAName(self.create_error_message("", "", None))
    }

    pub(super) fn invalid_record_member_error(&self) -> Error {
        Error::InvalidMember(self.create_error_message("", "", None))
    }
}
