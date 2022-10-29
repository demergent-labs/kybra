use crate::{
    errors::{ErrorMessage, Message},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub(super) fn multiple_targets_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: Message {
                title: todo!(),
                origin: todo!(),
                line_number: todo!(),
                source: todo!(),
                range: todo!(),
                annotation: todo!(),
                suggestion: todo!(),
            },
        }
    }
    pub(super) fn invalid_target_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: Message {
                title: todo!(),
                origin: todo!(),
                line_number: todo!(),
                source: todo!(),
                range: todo!(),
                annotation: todo!(),
                suggestion: todo!(),
            },
        }
    }
    pub(super) fn not_a_tuple_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: Message {
                title: todo!(),
                origin: todo!(),
                line_number: todo!(),
                source: todo!(),
                range: todo!(),
                annotation: todo!(),
                suggestion: todo!(),
            },
        }
    }
}
