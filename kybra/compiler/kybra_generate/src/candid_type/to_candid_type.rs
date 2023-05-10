use cdk_framework::act::node::CandidType;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

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
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    "Async" | "Manual" => {
                        SourceMapped::new(slice.as_ref(), self.source_map.clone()).to_candid_type()
                    }
                    _ => Err(InvalidSubscriptable::err_from_expr(self).into()),
                },
                _ => Err(InvalidSubscriptable::err_from_expr(self).into()),
            },
            ExprKind::Constant { value, .. } => match value {
                Constant::None => Err(NoneCannotBeAType::err_from_expr(self).into()),
                _ => Err(UnsupportedType::err_from_expr(self).into()),
            },
            _ => Err(UnsupportedType::err_from_expr(self).into()),
        }
    }
}