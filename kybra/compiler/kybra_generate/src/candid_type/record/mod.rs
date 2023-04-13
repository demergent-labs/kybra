mod record_members;

use cdk_framework::act::node::candid::Record;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};

impl PyAst {
    pub fn build_records(&self) -> Result<Vec<Record>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_record())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_record(&self) -> bool {
        match &self.node {
            StmtKind::ClassDef { bases, .. } => bases.iter().fold(false, |acc, base| {
                let is_record = match &base.node {
                    ExprKind::Name { id, .. } => id == "Record",
                    _ => false,
                };
                acc || is_record
            }),
            _ => false,
        }
    }

    pub fn as_record(&self) -> Result<Option<Record>, Vec<Error>> {
        if !self.is_record() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members = body
                    .iter()
                    .filter(|stmt| {
                        SourceMapped::new(stmt.clone(), self.source_map.clone()).is_record_member()
                    })
                    .map(|stmt| SourceMapped::new(stmt, self.source_map.clone()).to_record_member())
                    .collect_results()?;
                Ok(Some(Record {
                    name: Some(name.clone()),
                    members,
                    type_params: vec![].into(),
                }))
            }
            _ => Ok(None),
        }
    }
}
