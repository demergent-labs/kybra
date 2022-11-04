use crate::{errors::ErrorMessage, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub(super) fn not_a_record_error(&self) -> ErrorMessage {
        todo!()
    }
}
