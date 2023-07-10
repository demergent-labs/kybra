use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{
    candid_type::{errors::InvalidMember, warnings::DefaultValueIgnored},
    source_map::SourceMapped,
    Error,
};
use cdk_framework::{act::node::candid::record::Member, traits::CollectResults};

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
                annotation, value, ..
            } => {
                match value {
                    Some(_) => eprintln!("{}", DefaultValueIgnored::new(self)),
                    None => (),
                }
                let (name, candid_type) = (
                    self.get_name_or_err().map_err(Error::into),
                    SourceMapped::new(annotation.as_ref(), self.source_map.clone())
                        .to_candid_type(),
                )
                    .collect_results()?;
                Ok(Some(Member {
                    name: name.to_string(),
                    candid_type,
                }))
            }
            _ => Err(InvalidMember::err_from_stmt(self).into()),
        }
    }
}
