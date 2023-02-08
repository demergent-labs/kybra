mod canister_method;
mod data_types;
mod external_canisters;
pub mod function_guards;
mod get_dependencies;
mod get_source_info;
pub mod stable_b_tree_map_nodes;
mod system_methods;
mod type_alias;

use rustpython_parser::ast::{Located, Mod, StmtKind};

use crate::source_map::SourceMap;

use super::KybraExpr;

#[derive(Clone)]
pub struct KybraStmt<'a> {
    pub stmt_kind: &'a Located<StmtKind>,
    pub programs: &'a Vec<Mod>,
    pub source_map: &'a SourceMap,
}

// TODO what is the difference if any to get_alias_name and get_name?
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
                        programs: self.programs,
                        source_map: self.source_map,
                    }
                    .get_name()
                }
            }
            StmtKind::AugAssign { .. } => todo!(),
            StmtKind::AnnAssign { target, .. } => KybraExpr {
                located_expr: target,
                programs: self.programs,
                source_map: self.source_map,
            }
            .get_name(),
            _ => None,
        }
    }
}
