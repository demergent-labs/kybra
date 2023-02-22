use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{py_ast::kybra_types::KybraStmt, source_map::SourceMapped};
use cdk_framework::act::node::data_type::variant::{Member, Variant};

mod errors;
mod variants_members;

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_variant(&self) -> Option<Variant> {
        match &self.node.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<Member> = body
                    .iter()
                    .map(|stmt| {
                        KybraStmt {
                            stmt_kind: stmt,
                            source_map: self.source_map.clone(),
                        }
                        .as_variant_member()
                    })
                    .collect();
                Some(Variant {
                    name: Some(name.clone()),
                    members,
                })
            }
            _ => panic!("{}", self.not_a_variant_error()),
        }
    }

    pub fn is_variant(&self) -> bool {
        match &self.node.node {
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
