use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::{
    cdk_act::{
        nodes::data_type_nodes::{
            act_record::{Record, RecordTypeAlias},
            ActRecord, ActRecordMember, LiteralOrTypeAlias,
        },
        ActDataType,
    },
    py_ast::kybra_types::KybraStmt,
};

mod errors;
mod record_members;

impl KybraStmt<'_> {
    pub fn as_record(&self) -> ActDataType {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<ActRecordMember> = body
                    .iter()
                    .map(|stmt| {
                        KybraStmt {
                            stmt_kind: stmt,
                            source_map: self.source_map,
                        }
                        .as_record_member()
                    })
                    .collect();
                ActDataType::Record(ActRecord {
                    act_type: LiteralOrTypeAlias::TypeAlias(RecordTypeAlias {
                        record: Record {
                            name: name.clone(),
                            members,
                        },
                    }),
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