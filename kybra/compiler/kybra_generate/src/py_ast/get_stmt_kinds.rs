use rustpython_parser::ast::{Located, Mod, StmtKind};
use std::ops::Deref;

use crate::source_map::SourceMapped;

use super::PyAst;

impl PyAst {
    pub fn get_stmt_kinds(&self) -> Vec<SourceMapped<&Located<StmtKind>>> {
        self.source_mapped_mods
            .iter()
            .fold(vec![], |acc, source_mapped_mod| {
                let source_mapped_stmt_kinds = match &source_mapped_mod.deref() {
                    Mod::Module { body, .. } => body
                        .iter()
                        .map(|stmt_kind| {
                            SourceMapped::new(stmt_kind, source_mapped_mod.source_map.clone())
                        })
                        .collect(),
                    _ => vec![],
                };
                vec![acc, source_mapped_stmt_kinds].concat()
            })
    }
}
