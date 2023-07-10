use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    candid_type::{errors::InvalidMember, warnings::DefaultValueIgnored},
    source_map::SourceMapped,
    Error,
};
use cdk_framework::{act::node::candid::variant::Member, traits::CollectResults};

impl SourceMapped<&Located<StmtKind>> {
    pub fn to_variant_member(&self) -> Result<Member, Vec<Error>> {
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
                Ok(Member {
                    name: name.to_string(),
                    candid_type,
                })
            }
            _ => Err(InvalidMember::err_from_stmt(self).into()),
        }
    }
}
