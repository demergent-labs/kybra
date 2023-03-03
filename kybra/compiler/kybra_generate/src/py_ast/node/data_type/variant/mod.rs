use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::act::node::data_type::variant::Variant;

mod errors;
mod variants_members;

impl PyAst {
    pub fn build_variants(&self) -> KybraResult<Vec<Variant>> {
        let mut variants = vec![];
        let mut error_messages = vec![];

        self.get_stmt_kinds()
            .iter()
            .for_each(|stmt_kind| match stmt_kind.as_variant() {
                Ok(Some(variant)) => variants.push(variant),
                Ok(None) => (),
                Err(errors) => error_messages.extend(errors),
            });

        if error_messages.is_empty() {
            Ok(variants)
        } else {
            Err(error_messages)
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_variant(&self) -> KybraResult<Option<Variant>> {
        if !self.is_variant() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let member_results: Vec<_> = body
                    .iter()
                    .map(|stmt| {
                        SourceMapped::new(stmt, self.source_map.clone()).as_variant_member()
                    })
                    .collect();
                Ok(Some(Variant {
                    name: Some(name.clone()),
                    members: crate::errors::collect_kybra_results(member_results)?,
                }))
            }
            _ => Err(self.not_a_variant_error()),
        }
    }

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
}
