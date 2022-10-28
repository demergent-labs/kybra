mod canister_method;
mod data_types;

use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMap;

pub struct KybraStmt<'a> {
    pub stmt_kind: &'a Located<StmtKind>,
    pub source_map: &'a SourceMap,
}
