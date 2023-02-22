use cdk_framework::{act::node::data_type::TypeAlias, ToDataType};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::source_map::SourceMapped;

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_type_alias(&self) -> bool {
        match &self.node.node {
            StmtKind::Assign { value, .. } => {
                SourceMapped {
                    node: value.as_ref(),
                    source_map: self.source_map.clone(),
                }
                .is_data_type()
                    && false // TODO fix this
            }
            StmtKind::AnnAssign { annotation, .. } => match &annotation.node {
                ExprKind::Name { id, .. } => id == "TypeAlias",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_type_alias(&self) -> Option<TypeAlias> {
        if !self.is_type_alias() {
            return None;
        }
        let (alias_name, value) = match &self.node.node {
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
        let enclosed_type = SourceMapped {
            node: value.as_ref(),
            source_map: self.source_map.clone(),
        }
        .to_data_type();
        Some(TypeAlias {
            name: alias_name,
            aliased_type: Box::new(enclosed_type),
        })
    }
}
