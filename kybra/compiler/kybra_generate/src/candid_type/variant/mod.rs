use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};
use cdk_framework::act::node::candid::variant::Variant;

mod errors;
mod variants_members;

impl PyAst {
    pub fn build_variants(&self) -> Result<Vec<Variant>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_variant())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn is_variant(&self) -> bool {
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

    fn as_variant(&self) -> Result<Option<Variant>, Vec<Error>> {
        if !self.is_variant() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members = body
                    .iter()
                    .map(|stmt| {
                        SourceMapped::new(stmt, self.source_map.clone()).as_variant_member()
                    })
                    .collect_results()?;
                Ok(Some(Variant {
                    name: Some(name.clone()),
                    members,
                    type_params: vec![].into(),
                }))
            }
            _ => Err(Unreachable::new_err().into()),
        }
    }
}
