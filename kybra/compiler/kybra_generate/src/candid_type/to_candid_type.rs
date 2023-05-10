use cdk_framework::act::node::CandidType;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{
    constants::{ASYNC, MANUAL},
    get_name::HasName,
    source_map::SourceMapped,
    Error,
};

use super::errors::{InvalidSubscriptable, NoneCannotBeAType, UnsupportedType};

impl SourceMapped<&Located<ExprKind>> {
    pub fn to_candid_type(&self) -> Result<CandidType, Vec<Error>> {
        if let Some(primitive) = self.as_primitive().map_err(Into::<Vec<Error>>::into)? {
            return Ok(CandidType::Primitive(primitive));
        }
        if let Some(array) = self.as_array()? {
            return Ok(CandidType::Array(array));
        }
        if let Some(opt) = self.as_opt()? {
            return Ok(CandidType::Opt(opt));
        }
        if let Some(tuple) = self.as_tuple(None)? {
            return Ok(CandidType::Tuple(tuple));
        }
        if let Some(type_ref) = self.as_type_ref() {
            return Ok(CandidType::TypeRef(type_ref));
        }
        if let Some(func) = self.as_func(None)? {
            return Ok(CandidType::Func(func));
        }
        if let ExprKind::Subscript { value, slice, .. } = &self.node {
            match value.get_name() {
                Some(ASYNC | MANUAL) => {
                    return SourceMapped::new(slice.as_ref(), self.source_map.clone())
                        .to_candid_type()
                }
                _ => return Err(InvalidSubscriptable::err_from_expr(self).into()),
            }
        }
        if let ExprKind::Constant { value, .. } = &self.node {
            if let Constant::None = value {
                return Err(NoneCannotBeAType::err_from_expr(self).into());
            }
        }
        Err(UnsupportedType::err_from_expr(self).into())
    }
}
