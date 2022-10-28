use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::{
    cdk_act::{nodes::data_type_nodes::ActRecordMember, ToActDataType},
    py_ast::kybra_types::{KybraExpr, KybraStmt},
};

mod errors;
mod warnings;

impl KybraStmt<'_> {
    pub fn as_record_member(&self) -> ActRecordMember {
        match &self.stmt_kind.node {
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                ..
            } => {
                match value {
                    Some(_) => eprintln!("{}", self.default_value_warning()),
                    None => (),
                }
                let member_name = match &target.node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.target_must_be_a_name_error()),
                };
                let member_type = KybraExpr {
                    located_expr: &annotation,
                    source_map: self.source_map,
                }
                .to_act_data_type(&None);
                ActRecordMember {
                    member_name,
                    member_type,
                }
            }
            _ => panic!("{}", self.invalid_record_member_error()),
        }
    }
}
