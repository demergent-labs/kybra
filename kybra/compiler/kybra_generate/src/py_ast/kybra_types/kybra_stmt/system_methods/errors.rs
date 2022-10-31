use crate::{errors::ErrorMessage, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub fn not_an_init_method_error(&self) -> ErrorMessage {
        ErrorMessage { message: todo!() }
    }
    pub fn missing_type_annotation_error(&self) -> ErrorMessage {
        ErrorMessage { message: todo!() }
    }
}
