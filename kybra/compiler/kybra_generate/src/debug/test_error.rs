use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    candid_type::errors::InvalidName,
    source_map::{GetSourceInfo, SourceMapped},
    Error,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn test_error(&self) -> Error {
        eprintln!("FINAL RANGE: {:?}", self.get_range());
        // eprintln!("ALL TOKENS: {:?}", self.source_map.token_lines);
        eprintln!("FINAL SOURCE: {:?}", self.get_source());
        eprintln!("FINAL TEXT: {:?}", self.get_text());
        // panic!("Lets panic right here so we don't run into any problems");
        InvalidName::err_from_stmt(self)
    }
}
