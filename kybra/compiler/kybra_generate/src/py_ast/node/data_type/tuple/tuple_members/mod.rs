use cdk_framework::act::node::data_type::tuple::Member;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn as_tuple_member(&self) -> KybraResult<Member> {
        Ok(Member {
            type_: self.to_data_type()?,
        })
    }
}
