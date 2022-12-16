use crate::{
    errors::{CreateMessage, ErrorMessage},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub fn multiple_targets_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: self.create_message("", "", None),
        }
    }

    pub fn invalid_target_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: self.create_message("", "", None),
        }
    }

    pub fn not_a_tuple_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: self.create_message("", "", None),
        }
    }
}
