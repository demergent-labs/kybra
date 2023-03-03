mod record_members;

use cdk_framework::act::node::data_type::Record;
use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_records(&self) -> KybraResult<Vec<Record>> {
        let mut records = vec![];
        let mut error_messages = vec![];

        self.get_stmt_kinds()
            .iter()
            .for_each(|stmt_kind| match stmt_kind.as_record() {
                Ok(Some(record)) => records.push(record),
                Ok(None) => (),
                Err(errors) => error_messages.extend(errors),
            });

        if error_messages.is_empty() {
            Ok(records)
        } else {
            Err(error_messages)
        }
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

    pub fn as_record(&self) -> KybraResult<Option<Record>> {
        if !self.is_record() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<_> = crate::errors::collect_kybra_results(
                    body.iter()
                        .filter(|stmt| match &stmt.node {
                            StmtKind::Expr { value } => match &value.node {
                                ExprKind::Constant { value, .. } => match value {
                                    // Remove ellipses since they are parsed as nothing.
                                    Constant::Ellipsis => false,
                                    _ => true,
                                },
                                _ => true,
                            },
                            _ => true,
                        })
                        .map(|stmt| {
                            SourceMapped::new(stmt, self.source_map.clone()).as_record_member()
                        })
                        .collect(),
                )?;
                Ok(Some(Record {
                    name: Some(name.clone()),
                    members,
                }))
            }
            _ => Ok(None),
        }
    }
}
