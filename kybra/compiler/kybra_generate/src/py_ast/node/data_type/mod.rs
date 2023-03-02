use cdk_framework::act::node::DataType;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::source_map::SourceMapped;

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
    pub fn to_data_type(&self) -> DataType {
        // TODO make these ifs that return instead of else if
        if self.is_primitive() {
            return match self.to_primitive() {
                Ok(primitive) => DataType::Primitive(primitive),
                Err(error) => panic!("{}", error),
            };
        }
        if self.is_array() {
            return match self.to_array() {
                Ok(array) => DataType::Array(array),
                Err(error) => panic!("{}", error),
            };
        }
        if self.is_opt() {
            return match self.to_opt() {
                Ok(opt) => DataType::Opt(opt),
                Err(error) => panic!("{}", error),
            };
        }
        if self.is_tuple() {
            return match self.to_tuple(None) {
                Ok(tuple) => DataType::Tuple(tuple),
                Err(error) => panic!("{}", error),
            };
        }
        if self.is_type_ref() {
            return match self.to_type_ref() {
                Ok(type_ref) => DataType::TypeRef(type_ref),
                Err(error) => panic!("{}", error),
            };
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => match &id[..] {
                        "Async" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .to_data_type(),
                        "manual" => SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .to_data_type(),
                        _ => panic!("{}", self.invalid_subscript_value_error()),
                    },
                    _ => panic!("{}", self.invalid_subscript_value_error()),
                }
            }
            ExprKind::Constant { value, .. } => match value {
                Constant::None => {
                    panic!("{}", self.none_cant_be_a_type_error());
                }
                _ => {
                    todo!("{}", self.unsupported_type_error())
                }
            },
            _ => {
                panic!("{}", self.unsupported_type_error());
            }
        }
    }
}
