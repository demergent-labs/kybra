use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::Message, source_map::SourceMapped};

impl SourceMapped<&Located<StmtKind>> {
    pub fn not_a_function_def_error(&self) -> Vec<Message> {
        // TODO I'm guessing this error will mostly be for us if we use this
        // function wrong. If the function is used in the right place at the
        // right time then this should be unreachable".to_string(),
        todo!()
    }

    pub fn missing_type_annotation_error(&self) -> Message {
        todo!()
    }
}
