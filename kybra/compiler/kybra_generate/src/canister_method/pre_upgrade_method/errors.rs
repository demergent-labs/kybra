use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_pre_upgrade_method_allowed_error(&self) -> Error {
        Error::MultiplePreUpgrade(self.create_error_message(
            "Only one pre upgrade message allowed",
            "Duplicate pre upgrade here",
            None,
        ))
    }

    pub fn pre_upgrade_method_must_return_void_error(&self) -> Error {
        Error::ReturnTypeMustBeVoid(self.create_error_message(
            "PreUpgrade method must have an explicit void return type annotation",
            "",
            None,
        ))
    }
}
