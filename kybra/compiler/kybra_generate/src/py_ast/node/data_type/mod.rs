use cdk_framework::act::node::DataType;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub mod array;
pub mod errors;
pub mod func;
pub mod opt;
pub mod primitive;
pub mod record;
pub mod tuple;
pub mod type_alias;
pub mod type_ref;
pub mod variant;

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_data_type(&self) -> bool {
        self.is_primitive()
            || self.is_array()
            || self.is_opt()
            || self.is_tuple()
            || self.is_type_ref()
            || self.is_subscript_slice_data_type()
    }

    fn is_subscript_slice_data_type(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => match &id[..] {
                        "Async" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .is_data_type(),
                        "manual" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .is_data_type(),
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
    pub fn to_data_type(&self) -> KybraResult<DataType> {
        if self.is_primitive() {
            return Ok(DataType::Primitive(self.to_primitive()?));
        }
        if self.is_array() {
            return Ok(DataType::Array(self.to_array()?));
        }
        if self.is_opt() {
            return Ok(DataType::Opt(self.to_opt()?));
        }
        if self.is_tuple() {
            return Ok(DataType::Tuple(self.to_tuple(None)?));
        }
        if self.is_type_ref() {
            return Ok(DataType::TypeRef(self.to_type_ref()?));
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => match &id[..] {
                        "Async" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .to_data_type(),
                        "manual" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .to_data_type(),
                        _ => Err(self.invalid_subscript_value_error()),
                    },
                    _ => Err(self.invalid_subscript_value_error()),
                }
            }
            ExprKind::Constant { value, .. } => match value {
                Constant::None => Err(self.none_cant_be_a_type_error()),
                _ => Err(self.unsupported_type_error()),
            },
            _ => Err(self.unsupported_type_error()),
        }
    }
}
