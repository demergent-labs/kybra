use cdk_framework::{act::node::data_type::TypeAlias, ToDataType};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_type_aliases(&self) -> Vec<TypeAlias> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_type_alias())
            .collect()
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_type_alias(&self) -> bool {
        match &self.node.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    "alias" => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_type_alias(&self) -> bool {
        if self.is_func() {
            return false;
        }
        match &self.node.node {
            StmtKind::Assign { value, .. } => SourceMapped {
                node: value.as_ref(),
                source_map: self.source_map.clone(),
            }
            .is_type_alias(),
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
                let value = match &value.node {
                    ExprKind::Subscript { slice, .. } => slice,
                    _ => todo!(),
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
