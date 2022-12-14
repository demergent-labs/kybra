use rustpython_parser::ast::{Constant, ExprKind, StmtKind};

use crate::py_ast::kybra_types::KybraStmt;
use cdk_framework::act::node::data_type::{
    record::{Member, Record},
    DataType,
};

mod errors;
mod record_members;

impl KybraStmt<'_> {
    pub fn as_record(&self) -> DataType {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<Member> = body
                    .iter()
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
                        KybraStmt {
                            stmt_kind: stmt,
                            source_map: self.source_map.clone(),
                        }
                        .as_record_member()
                    })
                    .collect();
                DataType::Record(Record {
                    name: Some(name.clone()),
                    members,
                })
            }
            _ => panic!("{}", self.not_a_record_error()),
        }
    }

    pub fn is_record(&self) -> bool {
        match &self.stmt_kind.node {
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
}
