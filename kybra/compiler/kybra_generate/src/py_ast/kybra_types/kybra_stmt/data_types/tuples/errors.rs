use crate::{errors::ErrorMessage, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub(super) fn multiple_targets_error(&self) -> ErrorMessage {
        todo!()
    }

    pub(super) fn invalid_target_error(&self) -> ErrorMessage {
        todo!()
    }

    pub(super) fn not_a_tuple_error(&self) -> ErrorMessage {
        todo!()
    }
}
