use crate::{
    errors::{ErrorMessage, Message},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub fn not_a_function_def_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: Message {
                title: "I'm guessing this error will mostly be for us if we use this function wrong. If the function is used in the right place at the right time then this should be unreachable".to_string(),
                origin: todo!(),
                line_number: todo!(),
                source: todo!(),
                range: todo!(),
                annotation: todo!(),
                suggestion: todo!(),
            },
        }
    }
    pub fn missing_type_annotation_error(&self) -> ErrorMessage {
        ErrorMessage { message: todo!() }
    }
}
