use cdk_framework::act::node::candid::Array;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    fn get_array(&self) -> Option<&Located<ExprKind>> {
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match id == "Vec" {
                    true => Some(slice.as_ref()),
                    false => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_array(&self) -> Result<Option<Array>, Vec<Error>> {
        match self.get_array() {
            Some(array) => Ok(Some(Array {
                enclosed_type: Box::from(
                    SourceMapped::new(array, self.source_map.clone()).to_candid_type()?,
                ),
            })),
            None => Ok(None),
        }
    }
}
