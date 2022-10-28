use crate::{
    errors::{ErrorMessage, Message},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub(super) fn variant_target_must_be_a_name_error(&self) -> ErrorMessage {
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

    pub(super) fn invalid_variant_member_error(&self) -> ErrorMessage {
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
