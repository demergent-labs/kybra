use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

// TODO many of these errors are also used by type alias. If they are the same errors they should live somewhere were they both can access them. If not they we need to duplicate them for type alias and make them unique
impl SourceMapped<&Located<StmtKind>> {
    pub fn multiple_targets_error(&self) -> Vec<Message> {
        vec![self.create_error_message("", "", None)]
    }

    pub fn invalid_target_error(&self) -> Vec<Message> {
        vec![self.create_error_message("", "", None)]
    }

    pub fn not_a_tuple_error(&self) -> Vec<Message> {
        vec![self.create_error_message("", "", None)]
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_tuple_error(&self) -> Vec<Message> {
        vec![self.create_error_message("This is is not a tuple", "", None)]
    }
}