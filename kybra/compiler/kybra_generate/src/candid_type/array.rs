use cdk_framework::act::node::candid::Array;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::Unreachable, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    fn is_array(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Vec",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_array(&self) -> Result<Option<Array>, Vec<Error>> {
        if !self.is_array() {
            return Ok(None);
        }
        Ok(Some(match &self.node {
            ExprKind::Subscript { slice, .. } => Array {
                enclosed_type: Box::from(
                    SourceMapped::new(slice.as_ref(), self.source_map.clone()).to_candid_type()?,
                ),
            },
            _ => return Err(Unreachable::error().into()),
        }))
    }
}
