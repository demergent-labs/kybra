use cdk_framework::act::node::candid::tuple::Member;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn as_tuple_member(&self) -> KybraResult<Member> {
        Ok(Member {
            candid_type: self.to_data_type()?,
        })
    }
}
