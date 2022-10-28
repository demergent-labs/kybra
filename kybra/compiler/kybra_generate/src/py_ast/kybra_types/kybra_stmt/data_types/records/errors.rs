use crate::{errors::ErrorMessage, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub(super) fn not_a_record_error(&self) -> ErrorMessage {
        ErrorMessage {
            title: todo!(),
            origin: todo!(),
            line_number: todo!(),
            source: todo!(),
            range: todo!(),
            annotation: todo!(),
            suggestion: todo!(),
        }
    }
}
