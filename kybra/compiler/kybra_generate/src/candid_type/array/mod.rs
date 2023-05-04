use cdk_framework::act::node::candid::Array;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_array(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Vec",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_array(&self) -> Result<Array, Vec<Error>> {
        if !self.is_array() {
            return Err(crate::errors::unreachable().into());
        }
        match &self.node {
            ExprKind::Subscript { slice, .. } => Ok(Array {
                enclosed_type: Box::from(
                    SourceMapped::new(slice.as_ref(), self.source_map.clone()).to_candid_type()?,
                ),
            }),
            _ => Err(vec![crate::errors::unreachable()]),
        }
    }
}
