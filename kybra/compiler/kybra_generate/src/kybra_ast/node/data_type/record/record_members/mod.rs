use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::source_map::SourceMapped;
use cdk_framework::{act::node::data_type::record::Member, ToDataType};

mod errors;
mod warnings;

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_record_member(&self) -> Member {
        match &self.node.node {
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
                let name = match &target.node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.record_target_must_be_a_name_error()),
                };
                let type_ = SourceMapped {
                    node: annotation.as_ref(),
                    source_map: self.source_map.clone(),
                }
                .to_data_type();
                Member { name, type_ }
            }
            _ => panic!("{}", self.invalid_record_member_error()),
        }
    }
}
