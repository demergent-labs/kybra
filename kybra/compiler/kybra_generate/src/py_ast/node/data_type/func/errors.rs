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
    pub fn only_one_func_per_func_decl_error(&self) -> Vec<Message> {
        vec![self.create_error_message("You need to have Func([]) not Func([],[])", "", None)]
    }

    pub fn return_type_mode_error(&self) -> Vec<Message> {
        vec![self.create_error_message("return type must be oneway, query, or update", "", None)]
    }

    pub fn todo_func_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Not a func", "", None)]
    }

    pub fn todo_func_small_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Not a func", "", None)]
    }

    pub fn not_a_func_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Not a func", "", None)]
    }

    pub fn inline_func_not_supported(&self) -> Vec<Message> {
        vec![self.create_error_message("Inline Func Not supported", "", None)]
    }
}
