use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::{GetSourceInfo, SourceMapped},
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn test_error(&self) -> Message {
        eprintln!("FINAL RANGE: {:?}", self.get_range());
        // eprintln!("ALL TOKENS: {:?}", self.source_map.token_lines);
        eprintln!("FINAL SOURCE: {:?}", self.get_source());
        eprintln!("FINAL TEXT: {:?}", self.get_text());
        // panic!("Lets panic right here so we don't run into any problems");
        self.create_error_message("This is a test error", "Test is here", None)
    }
}
