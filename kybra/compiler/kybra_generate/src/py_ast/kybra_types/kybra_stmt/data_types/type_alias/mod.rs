use cdk_framework::{
    act::node::{data_type::TypeAlias, DataType},
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::{KybraExpr, KybraStmt};

impl KybraStmt<'_> {
    pub fn is_type_alias(&self) -> bool {
        !(self.is_record() || self.is_tuple() || self.is_variant() || self.is_func()) // TODO will any of these actually possible look like a type alias?
            && match &self.stmt_kind.node {
                StmtKind::Assign { value: _, .. } => {
                    // TODO we should make sure that the thing on the other end of the assign is able to be a type
                    true
                }
                StmtKind::AnnAssign { annotation, .. } => match &annotation.node {
                    ExprKind::Name { id, .. } => id == "TypeAlias",
                    _ => false,
                },
                _ => false,
            }
    }

    pub fn as_type_alias(&self) -> DataType {
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
        let enclosed_type = KybraExpr {
            located_expr: value,
            source_map: self.source_map.clone(),
        }
        .to_data_type();
        DataType::TypeAlias(TypeAlias {
            name: alias_name,
            aliased_type: Box::new(enclosed_type),
        })
    }
}
