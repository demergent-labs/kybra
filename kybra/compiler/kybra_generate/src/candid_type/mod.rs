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
    pub fn is_candid_type(&self) -> bool {
        self.is_primitive()
            || self.is_array()
            || self.is_opt()
            || self.is_tuple()
            || self.is_type_ref()
            || self.is_subscript_slice_a_candid_type()
    }

    fn is_subscript_slice_a_candid_type(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => match &id[..] {
                        "Async" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .is_candid_type(),
                        "Manual" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .is_candid_type(),
                        _ => false,
                    },
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn to_candid_type(&self) -> Result<CandidType, Vec<Error>> {
        if self.is_primitive() {
            return match self.to_primitive() {
                Ok(primitive) => Ok(CandidType::Primitive(primitive)),
                Err(err) => Err(vec![err]),
            };
        }
        if self.is_array() {
            return Ok(CandidType::Array(self.to_array()?));
        }
        if self.is_opt() {
            return Ok(CandidType::Opt(self.to_opt()?));
        }
        if self.is_tuple() {
            return Ok(CandidType::Tuple(self.to_tuple(None)?));
        }
        if self.is_type_ref() {
            return match self.to_type_ref() {
                Ok(type_ref) => Ok(CandidType::TypeRef(type_ref)),
                Err(err) => Err(vec![err]),
            };
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
