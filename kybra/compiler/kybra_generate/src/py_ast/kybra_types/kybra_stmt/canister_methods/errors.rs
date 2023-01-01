use crate::{
    errors::{CreateMessage, ErrorMessage},
    py_ast::kybra_types::KybraStmt,
    source_map::GetSourceInfo,
};

impl KybraStmt<'_> {
    pub fn test_error(&self) -> ErrorMessage {
        eprintln!("FINAL RANGE: {:?}", self.get_range());
        // eprintln!("ALL TOKENS: {:?}", self.source_map.token_lines);
        eprintln!("FINAL SOURCE: {:?}", self.get_source());
        eprintln!("FINAL TEXT: {:?}", self.get_text());
        // panic!("Lets paic right here so we don't run into any problems");
        self.create_error_message("This is a test error", "Test is here", None)
    }
}
