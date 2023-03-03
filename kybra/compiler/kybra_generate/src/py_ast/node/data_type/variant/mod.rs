use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::act::node::data_type::variant::Variant;

mod errors;
mod variants_members;

impl PyAst {
    pub fn build_variants(&self) -> KybraResult<Vec<Variant>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_variant())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_variant(&self) -> bool {
        match &self.node {
            StmtKind::ClassDef { bases, .. } => bases.iter().fold(false, |acc, base| {
                let is_variant = match &base.node {
                    ExprKind::Name { id, .. } => id == "Variant",
                    _ => false,
                };
                acc || is_variant
            }),
            _ => false,
        }
    }

    pub fn as_variant(&self) -> KybraResult<Option<Variant>> {
        if !self.is_variant() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<_> = crate::errors::collect_kybra_results(
                    body.iter()
                        .map(|stmt| {
                            SourceMapped::new(stmt, self.source_map.clone()).as_variant_member()
                        })
                        .collect(),
                )?;
                Ok(Some(Variant {
                    name: Some(name.clone()),
                    members,
                }))
            }
            _ => Err(self.not_a_variant_error()),
        }
    }
}
