use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn variant_target_must_be_a_name_error(&self) -> Vec<Message> {
        vec![self.create_error_message("variant target must be a name", "", None)]
    }

    pub fn invalid_variant_member_error(&self) -> Vec<Message> {
        vec![self.create_error_message("invalid variant member", "", None)]
    }
}
