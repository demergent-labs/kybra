use cdk_framework::act::node::candid::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn as_opt(&self) -> Result<Option<Opt>, Vec<Error>> {
        match self.get_subscript_slice_for("Opt") {
            Some(opt) => Ok(Some(Opt {
                enclosed_type: Box::from(
                    SourceMapped::new(opt, self.source_map.clone()).to_candid_type()?,
                ),
            })),
            None => Ok(None),
        }
    }
}
