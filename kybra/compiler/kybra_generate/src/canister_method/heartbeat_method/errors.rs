use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_heartbeat_allowed_error(&self) -> Error {
        Error::MultipleHeartBeat(self.create_error_message(
            "Only one heartbeat function allowed",
            "Duplicate heartbeat here",
            None,
        ))
    }

    pub fn heartbeat_method_must_return_void_error(&self) -> Error {
        Error::ReturnTypeMustBeVoid(self.create_error_message(
            "Heartbeat method must have an explicit void return type annotation",
            "",
            None,
        ))
    }
}
