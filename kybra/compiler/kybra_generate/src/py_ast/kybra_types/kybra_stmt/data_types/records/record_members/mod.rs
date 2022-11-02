use rustpython_parser::ast::{ExprKind, StmtKind};

use crate::py_ast::kybra_types::{KybraExpr, KybraStmt};
use cdk_framework::{nodes::data_type_nodes::ActRecordMember, ToActDataType};

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
                    Some(_) => eprintln!("{}", self.record_default_value_warning()),
                    None => (),
                }
                let member_name = match &target.node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.record_target_must_be_a_name_error()),
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
