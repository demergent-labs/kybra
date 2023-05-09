use cdk_framework::act::node::candid::Array;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn as_array(&self) -> Result<Option<Array>, Vec<Error>> {
        match self.get_subscript_slice_for("Vec") {
            Some(array) => Ok(Some(Array {
                enclosed_type: Box::from(
                    SourceMapped::new(array, self.source_map.clone()).to_candid_type()?,
                ),
            })),
            None => Ok(None),
        }
    }
}
