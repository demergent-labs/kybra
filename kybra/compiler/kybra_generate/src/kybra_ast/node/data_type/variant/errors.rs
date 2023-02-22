use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::Message, source_map::SourceMapped};

impl SourceMapped<'_, Located<StmtKind>> {
    pub(super) fn not_a_variant_error(&self) -> Message {
        todo!()
    }
}
