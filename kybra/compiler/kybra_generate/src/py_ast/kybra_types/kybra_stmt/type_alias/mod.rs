use rustpython_parser::ast::{ExprKind, StmtKind};

use super::KybraStmt;

// TODO what is the difference if any to get_alias_name and get_name?
impl KybraStmt<'_> {
    pub fn get_alias_name(&self) -> Option<String> {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { name, .. } => Some(name.clone()),
            StmtKind::Assign { targets, .. } => {
                if targets.len() != 1 {
                    return None;
                }
                match &targets[0].node {
                    ExprKind::Name { id, .. } => Some(id.clone()),
                    _ => None,
                }
            }
            StmtKind::AnnAssign { target, .. } => match &target.node {
                ExprKind::Name { id, .. } => Some(id.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}
