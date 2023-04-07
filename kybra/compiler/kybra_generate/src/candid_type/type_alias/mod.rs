pub mod errors;

use cdk_framework::act::node::candid::TypeAlias;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_type_aliases(&self) -> KybraResult<Vec<TypeAlias>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_type_alias())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_type_alias(&self) -> bool {
        match &self.node {
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
        match &self.node {
            StmtKind::Assign { value, .. } => {
                SourceMapped::new(value.as_ref(), self.source_map.clone()).is_type_alias()
            }
            _ => false,
        }
    }

    pub fn as_type_alias(&self) -> KybraResult<Option<TypeAlias>> {
        if !self.is_type_alias() {
            return Ok(None);
        }
        let (alias_name, value) = match &self.node {
            StmtKind::Assign { targets, value, .. } => {
                if targets.len() > 1 {
                    return Err(self.multiple_targets_error());
                }
                let alias_name = match &targets[0].node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => return Err(self.invalid_target_error()),
                };
                let value = match &value.node {
                    ExprKind::Subscript { slice, .. } => slice,
                    _ => return Err(self.must_be_subscript_error()),
                };
                (alias_name, value)
            }
            _ => return Ok(None),
        };
        let enclosed_type =
            match SourceMapped::new(value.as_ref(), self.source_map.clone()).to_candid_type() {
                Ok(enclosed_type) => enclosed_type,
                Err(err) => return Err(err),
            };
        Ok(Some(TypeAlias {
            name: alias_name,
            aliased_type: Box::new(enclosed_type),
            type_params: vec![].into(),
        }))
    }
}
