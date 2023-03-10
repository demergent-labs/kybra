use cdk_framework::act::node::candid::tuple::Elem;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn as_tuple_member(&self) -> KybraResult<Elem> {
        Ok(Elem {
            candid_type: self.to_candid_type()?,
        })
    }
}
