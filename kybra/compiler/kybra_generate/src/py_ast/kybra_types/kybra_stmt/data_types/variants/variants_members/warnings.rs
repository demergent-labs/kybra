use crate::{
    errors::{Message, WarningMessage},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub(super) fn variant_default_value_warning(&self) -> WarningMessage {
        WarningMessage {
            message: Message {
                title: "WARNING: I don't think default values are supported are they?".to_string(),
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
