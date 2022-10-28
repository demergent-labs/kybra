use crate::py_ast::kybra_types::KybraStmt;

impl KybraStmt<'_> {
    pub(super) fn default_value_warning(&self) -> String {
        "WARNING: I don't think default values are supported are they?".to_string()
    }
}
