pub mod errors;
mod record_members;

use cdk_framework::act::node::data_type::{record::Member, Record};
use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{errors::Message, source_map::SourceMapped};

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_record(&self) -> bool {
        match &self.node.node {
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

    pub fn to_record(&self) -> Result<Record, Message> {
        match self.as_record() {
            Some(record) => Ok(record),
            None => Err(self.not_a_record_error()),
        }
    }

    pub fn as_record(&self) -> Option<Record> {
        match &self.node.node {
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
                        SourceMapped {
                            node: stmt,
                            source_map: self.source_map.clone(),
                        }
                        .as_record_member()
                    })
                    .collect();
                Some(Record {
                    name: Some(name.clone()),
                    members,
                })
            }
            _ => None,
        }
    }
}
