use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::KybraStmt;
use cdk_framework::act::node::data_type::{
    variant::{ActVariantMember, Variant},
    DataType,
};

mod errors;
mod variants_members;

impl KybraStmt<'_> {
    pub fn as_variant(&self) -> DataType {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { name, body, .. } => {
                let members: Vec<ActVariantMember> = body
                    .iter()
                    .map(|stmt| {
                        KybraStmt {
                            stmt_kind: stmt,
                            source_map: self.source_map,
                        }
                        .as_variant_member()
                    })
                    .collect();
                DataType::Variant(Variant {
                    name: name.clone(),
                    members,
                })
            }
            _ => panic!("{}", self.not_a_variant_error()),
        }
    }

    pub fn is_variant(&self) -> bool {
        match &self.stmt_kind.node {
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
