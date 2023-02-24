use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub(super) fn not_a_record_error(&self) -> Message {
        self.create_error_message("Not a record", "", None)
    }
}
