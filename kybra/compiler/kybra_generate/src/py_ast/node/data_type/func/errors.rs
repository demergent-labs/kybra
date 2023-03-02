use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

// TODO this is a standin error because I need funcs to not have todo!() in them right now
impl SourceMapped<&Located<StmtKind>> {
    pub fn todo_func_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Not a func", "", None)]
    }
}
impl SourceMapped<&Located<ExprKind>> {
    pub fn todo_func_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Not a func", "", None)]
    }

    pub fn not_a_func_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Not a func", "", None)]
    }

    pub fn inline_func_not_supported(&self) -> Vec<Message> {
        vec![self.create_error_message("Inline Func Not supported", "", None)]
    }
}
