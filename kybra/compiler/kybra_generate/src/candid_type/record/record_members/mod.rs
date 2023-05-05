use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{source_map::SourceMapped, Error};
use cdk_framework::act::node::candid::record::Member;

mod errors;
mod warnings;

impl SourceMapped<&Located<StmtKind>> {
    // Ellipses can show up in records but are not record members, and should
    // not be parsed as such.
    fn is_ellipses_member(&self) -> bool {
        match &self.node {
            StmtKind::Expr { value } => match &value.node {
                ExprKind::Constant { value, .. } => match value {
                    Constant::Ellipsis => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_record_member(&self) -> Result<Option<Member>, Vec<Error>> {
        // Ignore ellipses
        if self.is_ellipses_member() {
            return Ok(None);
        }
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
                    _ => return Err(self.record_member_target_must_be_a_name_error().into()),
                };
                let candid_type = SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                    .to_candid_type()?;

                Ok(Some(Member { name, candid_type }))
            }
            _ => Err(self.invalid_record_member_error().into()),
        }
    }
}
