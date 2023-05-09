use cdk_framework::act::node::candid::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    fn get_opt(&self) -> Option<&Located<ExprKind>> {
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match id == "Opt" {
                    true => Some(slice.as_ref()),
                    false => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_opt(&self) -> Result<Option<Opt>, Vec<Error>> {
        match self.get_opt() {
            Some(opt) => Ok(Some(Opt {
                enclosed_type: Box::from(
                    SourceMapped::new(opt, self.source_map.clone()).to_candid_type()?,
                ),
            })),
            None => todo!(),
        }
    }
}
