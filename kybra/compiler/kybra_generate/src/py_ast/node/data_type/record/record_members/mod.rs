use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};
use cdk_framework::act::node::data_type::record::Member;

mod errors;
mod warnings;

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_record_member(&self) -> bool {
        // Ellipses can show up in records but are not record members.
        match &self.node {
            StmtKind::Expr { value } => match &value.node {
                ExprKind::Constant { value, .. } => match value {
                    Constant::Ellipsis => false,
                    _ => true,
                },
                _ => true,
            },
            _ => true,
        }
    }

    pub fn to_record_member(&self) -> KybraResult<Member> {
        match &self.node {
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
                    _ => return Err(self.record_target_must_be_a_name_error()),
                };
                let type_ = SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                    .to_data_type()?;
                Ok(Member { name, type_ })
            }
            _ => Err(self.invalid_record_member_error()),
        }
    }
}
