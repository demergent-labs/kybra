use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    candid_type::errors::{InvalidMember, TargetMustBeAName},
    source_map::SourceMapped,
    Error,
};
use cdk_framework::act::node::candid::variant::Member;

mod warnings;

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_variant_member(&self) -> Result<Member, Vec<Error>> {
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
                    _ => return Err(TargetMustBeAName::err_from_stmt(self).into()),
                };
                let candid_type = SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                    .to_candid_type()?;
                Ok(Member { name, candid_type })
            }
            _ => Err(InvalidMember::err_from_stmt(self).into()),
        }
    }
}
