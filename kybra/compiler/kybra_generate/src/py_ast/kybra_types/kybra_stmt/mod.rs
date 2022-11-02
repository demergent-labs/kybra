mod canister_method;
mod data_types;
mod get_dependencies;
mod system_methods;
mod type_alias;

use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMap;

use super::KybraExpr;

#[derive(Clone)]
pub struct KybraStmt<'a> {
    pub stmt_kind: &'a Located<StmtKind>,
    pub source_map: &'a SourceMap,
}

impl KybraStmt<'_> {
    pub fn get_name(&self) -> Option<String> {
        match &self.stmt_kind.node {
            StmtKind::FunctionDef { name, .. } => Some(name.clone()),
            StmtKind::AsyncFunctionDef { name, .. } => Some(name.clone()),
            StmtKind::ClassDef { name, .. } => Some(name.clone()),
            StmtKind::Assign { targets, .. } => {
                if targets.len() != 1 {
                    None
                } else {
                    KybraExpr {
                        located_expr: &targets[0],
                        source_map: self.source_map,
                    }
                    .get_name()
                }
            }
            StmtKind::AugAssign { .. } => todo!(),
            StmtKind::AnnAssign { .. } => todo!(),
            _ => None,
        }
    }

    pub fn is_canister(&self) -> bool {
        // TODO implement when the canister format is finalized
        false
    }
}
