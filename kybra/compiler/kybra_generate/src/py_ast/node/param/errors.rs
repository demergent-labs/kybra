use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn return_type_annotation_required_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Return type annotation required", "", None)]
    }
}
