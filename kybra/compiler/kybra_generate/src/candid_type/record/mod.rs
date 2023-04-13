mod record_members;

use cdk_framework::act::node::{candid::Record, Member};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_records(&self) -> KybraResult<Vec<Record>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_record())
                .collect(),
        )?
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

    pub fn get_members(&self, body: &Vec<Located<StmtKind>>) -> Vec<KybraResult<Member>> {
        body.iter()
            .filter(|stmt| {
                SourceMapped::new(stmt.clone(), self.source_map.clone()).is_record_member()
            })
            .map(|stmt| SourceMapped::new(stmt, self.source_map.clone()).to_record_member())
            .collect()
    }

    pub fn as_record(&self) -> KybraResult<Option<Record>> {
        if !self.is_record() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<_> = crate::errors::collect_kybra_results(self.get_members(body))?;
                Ok(Some(Record {
                    name: Some(name.clone()),
                    members,
                    type_params: vec![].into(),
                }))
            }
            _ => Ok(None),
        }
    }

    pub fn record_uses_type_ref(&self, name: &str) -> bool {
        self.get_members(body);
        false
    }
}
