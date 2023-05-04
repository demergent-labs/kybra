use cdk_framework::act::node::candid::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::Unreachable, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    fn is_opt(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Opt",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_opt(&self) -> Result<Option<Opt>, Vec<Error>> {
        if !self.is_opt() {
            return Ok(None);
        }
        Ok(Some(match &self.node {
            ExprKind::Subscript { slice, .. } => Opt {
                enclosed_type: Box::from(
                    SourceMapped::new(slice.as_ref(), self.source_map.clone()).to_candid_type()?,
                ),
            },
            _ => return Err(Unreachable::new_err().into()),
        }))
    }
}
