use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};
use cdk_framework::act::node::data_type::variant::Member;

mod errors;
mod warnings;

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_variant_member(&self) -> KybraResult<Member> {
        match &self.node {
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                ..
            } => {
                match value {
                    Some(_) => eprintln!("{}", self.variant_default_value_warning()),
                    None => (),
                }
                let name = match &target.node {
                    ExprKind::Name { id, .. } => id.clone(),
                    _ => panic!("{}", self.variant_target_must_be_a_name_error()),
                };
                let type_ = SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                    .to_data_type()?;
                Ok(Member { name, type_ })
            }
            _ => panic!("{}", self.invalid_variant_member_error()),
        }
    }
}
