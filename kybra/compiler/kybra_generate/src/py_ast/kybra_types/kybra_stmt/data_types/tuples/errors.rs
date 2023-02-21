use crate::{
    errors::{CreateMessage, Message},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub fn multiple_targets_error(&self) -> Message {
        Message::Error(self.create_message("", "", None))
    }

    pub fn invalid_target_error(&self) -> Message {
        Message::Error(self.create_message("", "", None))
    }

    pub fn not_a_tuple_error(&self) -> Message {
        Message::Error(self.create_message("", "", None))
    }
}
