use crate::{
    errors::{CreateMessage, ErrorMessage},
    py_ast::kybra_types::KybraStmt,
    source_map::GetSourceInfo,
};

impl KybraStmt<'_> {
    pub fn test_error(&self) -> ErrorMessage {
        eprintln!("{:?}", self.get_range());
        eprintln!("{:?}", self.source_map.token_lines);
        self.create_error_message("This is a test error", "Test is here", None)
    }
}
