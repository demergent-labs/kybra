use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn func_formatting_error(&self) -> Vec<Message> {
        vec![self.create_error_message("You need to have Func([]) and not any other way", "", None)]
    }

    pub fn return_type_mode_error(&self) -> Vec<Message> {
        vec![self.create_error_message("return type must be oneway, query, or update", "", None)]
    }

    pub fn inline_func_not_supported(&self) -> Vec<Message> {
        vec![self.create_error_message("Inline Func Not supported", "", None)]
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn multiple_func_targets_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Only one target is supported", "", None)]
    }
}
