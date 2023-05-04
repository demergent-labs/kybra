use cdk_framework::act::node::CandidType;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{source_map::SourceMapped, Error};

pub mod array;
pub mod errors;
pub mod func;
pub mod opt;
pub mod primitive;
pub mod record;
pub mod service;
pub mod tuple;
pub mod type_alias;
pub mod type_ref;
pub mod variant;

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
        if let Some(type_ref) = self.as_type_ref().map_err(Into::<Vec<Error>>::into)? {
            return Ok(CandidType::TypeRef(type_ref));
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    "Async" | "Manual" => {
                        SourceMapped::new(slice.as_ref(), self.source_map.clone()).to_candid_type()
                    }
                    _ => Err(vec![self.invalid_subscriptable_error()]),
                },
                _ => Err(vec![self.invalid_subscriptable_error()]),
            },
            ExprKind::Constant { value, .. } => match value {
                Constant::None => Err(vec![self.none_cant_be_a_type_error()]),
                _ => Err(vec![self.unsupported_type_error()]),
            },
            _ => Err(vec![self.unsupported_type_error()]),
        }
    }
}
