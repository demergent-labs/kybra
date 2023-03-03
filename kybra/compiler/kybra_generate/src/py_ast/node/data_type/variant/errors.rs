use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMapped;

impl SourceMapped<&Located<StmtKind>> {}
