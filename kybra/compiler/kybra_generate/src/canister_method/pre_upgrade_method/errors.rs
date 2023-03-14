use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn only_one_pre_upgrade_method_allowed_error(&self) -> Message {
        self.create_error_message(
            "Only one pre upgrade message allowed",
            "Duplicate pre upgrade here",
            None,
        )
    }
}
