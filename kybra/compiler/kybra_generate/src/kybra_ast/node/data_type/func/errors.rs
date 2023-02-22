use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

// TODO this is a standin error because I need funcs to not have todo!() in them right now
impl SourceMapped<'_, Located<StmtKind>> {
    pub(super) fn todo_func_error(&self) -> Message {
        self.create_error_message("Not a func", "", None)
    }
}
