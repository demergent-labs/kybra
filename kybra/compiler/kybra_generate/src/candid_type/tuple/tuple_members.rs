use cdk_framework::act::node::candid::tuple::Elem;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn as_tuple_member(&self) -> Result<Elem, Vec<Error>> {
        Ok(Elem {
            candid_type: self.to_candid_type()?,
        })
    }
}
