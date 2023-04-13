use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn variant_target_must_be_a_name_error(&self) -> Error {
        Error::TargetMustBeAName(self.create_error_message(
            "variant target must be a name",
            "",
            None,
        ))
    }

    pub fn invalid_variant_member_error(&self) -> Error {
        Error::InvalidMember(self.create_error_message("invalid variant member", "", None))
    }
}
