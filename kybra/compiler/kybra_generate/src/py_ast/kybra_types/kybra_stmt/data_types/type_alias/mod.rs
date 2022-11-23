use cdk_framework::{ActDataType, ToActDataType};
use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::{KybraExpr, KybraStmt};

impl KybraStmt<'_> {
    pub fn is_type_alias(&self) -> bool {
        match &self.stmt_kind.node {
            StmtKind::Assign { .. } => true,
            StmtKind::AnnAssign { annotation, .. } => match &annotation.node {
                ExprKind::Name { id, .. } => id == "TypeAlias",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_type_alias(&self) -> ActDataType {
        let (alias_name, value) = match &self.stmt_kind.node {
            StmtKind::Assign { targets, value, .. } => {
                if targets.len() > 1 {
                    panic!("{}", self.multiple_targets_error())
                }
                let alias_name = match &targets[0].node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.invalid_target_error()),
                };
                (alias_name, value)
            }
            StmtKind::AnnAssign { target, value, .. } => {
                let alias_name = match &target.node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.invalid_target_error()),
                };
                let value = match value {
                    Some(value) => value,
                    None => todo!(),
                };
                (alias_name, value)
            }
            _ => panic!("This is not a type alias"),
        };
        KybraExpr {
            located_expr: value,
            source_map: self.source_map,
        }
        .to_act_data_type(&Some(&alias_name))
    }
}
