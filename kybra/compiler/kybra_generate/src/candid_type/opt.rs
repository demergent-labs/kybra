use cdk_framework::act::node::candid::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_opt(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Opt",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_opt(&self) -> Result<Opt, Vec<Error>> {
        if !self.is_opt() {
            return Err(crate::errors::unreachable().into());
        }
        match &self.node {
            ExprKind::Subscript { slice, .. } => Ok(Opt {
                enclosed_type: Box::from(
                    SourceMapped::new(slice.as_ref(), self.source_map.clone()).to_candid_type()?,
                ),
            }),
            _ => Err(crate::errors::unreachable().into()),
        }
    }
}
