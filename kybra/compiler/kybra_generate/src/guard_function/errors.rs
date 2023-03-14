use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn guard_functions_param_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Guards functions can't have parameters", "", None)]
    }

    pub fn guard_function_return_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Guard functions must return GuardResult", "", None)]
    }
}
