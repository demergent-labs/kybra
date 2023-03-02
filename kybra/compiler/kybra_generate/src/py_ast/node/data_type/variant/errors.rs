use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::Message, source_map::SourceMapped};

impl SourceMapped<&Located<StmtKind>> {
    pub(super) fn not_a_variant_error(&self) -> Vec<Message> {
        todo!()
    }
}
