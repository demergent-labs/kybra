use crate::{errors::Message, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub(super) fn record_target_must_be_a_name_error(&self) -> Message {
        todo!()
    }

    pub(super) fn invalid_record_member_error(&self) -> Message {
        todo!()
    }
}
