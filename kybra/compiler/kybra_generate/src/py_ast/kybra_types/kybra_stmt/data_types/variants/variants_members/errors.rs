use crate::{errors::ErrorMessage, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub(super) fn variant_target_must_be_a_name_error(&self) -> ErrorMessage {
        todo!()
    }

    pub(super) fn invalid_variant_member_error(&self) -> ErrorMessage {
        todo!()
    }
}
