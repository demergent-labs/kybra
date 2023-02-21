use crate::{errors::Message, py_ast::kybra_types::KybraStmt};

impl KybraStmt<'_> {
    pub(super) fn not_a_variant_error(&self) -> Message {
        todo!()
    }
}
