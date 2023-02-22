use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<'_, Located<StmtKind>> {
    pub(super) fn record_target_must_be_a_name_error(&self) -> Message {
        self.create_error_message("", "", None)
    }

    pub(super) fn invalid_record_member_error(&self) -> Message {
        self.create_error_message("", "", None)
    }
}
