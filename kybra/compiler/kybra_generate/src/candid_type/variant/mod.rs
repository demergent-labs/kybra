use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};
use cdk_framework::act::node::candid;

mod variants_members;

impl PyAst {
    pub fn build_variants(&self) -> Result<Vec<candid::Variant>, Vec<Error>> {
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
    fn as_variant(&self) -> Result<Option<candid::Variant>, Vec<Error>> {
        match self.get_child_class_of("Variant") {
            Some(variant) => {
                let members = variant
                    .body
                    .iter()
                    .map(|stmt| {
                        SourceMapped::new(stmt, self.source_map.clone()).to_variant_member()
                    })
                    .collect_results()?;
                Ok(Some(candid::Variant {
                    name: Some(variant.name.clone()),
                    members,
                    type_params: vec![].into(),
                }))
            }
            None => Ok(None),
        }
    }
}
