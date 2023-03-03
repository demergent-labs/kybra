use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::Message, source_map::SourceMapped};

impl SourceMapped<&Located<StmtKind>> {
    pub(super) fn variant_target_must_be_a_name_error(&self) -> Vec<Message> {
        todo!()
    }

    pub(super) fn invalid_variant_member_error(&self) -> Vec<Message> {
        todo!()
    }
}
