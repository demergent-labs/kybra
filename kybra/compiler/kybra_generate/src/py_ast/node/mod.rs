use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::source_map::SourceMapped;

pub mod canister_method;
pub mod data_type;
pub mod external_canister;
pub mod guard_function;
pub mod param;
pub mod stable_b_tree_map_nodes;

pub use stable_b_tree_map_nodes::StableBTreeMapNode;

impl SourceMapped<&Located<StmtKind>> {
    pub fn get_name(&self) -> Option<String> {
        match &self.node {
            StmtKind::FunctionDef { name, .. } => Some(name.clone()),
            StmtKind::AsyncFunctionDef { name, .. } => Some(name.clone()),
            StmtKind::ClassDef { name, .. } => Some(name.clone()),
            StmtKind::Assign { targets, .. } => {
                if targets.len() != 1 {
                    None
                } else {
                    SourceMapped::new(&targets[0], self.source_map.clone()).get_name()
                }
            }
            StmtKind::AnnAssign { target, .. } => {
                SourceMapped::new(target.as_ref(), self.source_map.clone()).get_name()
            }
            _ => None,
        }
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn get_name(&self) -> Option<String> {
        match &self.node {
            ExprKind::Name { id, .. } => Some(id.clone()),
            _ => None,
        }
    }
}
