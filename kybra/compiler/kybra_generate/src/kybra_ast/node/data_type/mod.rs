use cdk_framework::{act::node::DataType, ToDataType};
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

impl ToDataType for SourceMapped<'_, Located<ExprKind>> {
    fn to_data_type(&self) -> DataType {
        if self.is_primitive() {
            match self.to_primitive() {
                Ok(primitive) => DataType::Primitive(primitive),
                Err(error) => panic!("{}", error),
            }
        } else if self.is_array() {
            match self.to_array() {
                Ok(array) => DataType::Array(array),
                Err(error) => panic!("{}", error),
            }
        } else if self.is_opt() {
            match self.to_opt() {
                Ok(opt) => DataType::Opt(opt),
                Err(error) => panic!("{}", error),
            }
        } else if self.is_tuple() {
            match self.to_tuple(None) {
                Ok(tuple) => DataType::Tuple(tuple),
                Err(error) => panic!("{}", error),
            }
        } else if self.is_type_ref() {
            match self.to_type_ref() {
                Ok(type_ref) => DataType::TypeRef(type_ref),
                Err(error) => panic!("{}", error),
            }
        } else {
            match &self.node.node {
                ExprKind::Subscript { value, slice, .. } => match &value.node {
                    ExprKind::Name { id, .. } => match &id[..] {
                        "Async" => SourceMapped {
                            node: slice.as_ref(),
                            source_map: self.source_map.clone(),
                        }
                        .to_data_type(),
                        "manual" => SourceMapped {
                            node: slice.as_ref(),
                            source_map: self.source_map.clone(),
                        }
                        .to_data_type(),
                        _ => panic!("{}", self.invalid_subscript_value_error()),
                    },
                    _ => panic!("{}", self.invalid_subscript_value_error()),
                },
                ExprKind::Constant { value, .. } => match value {
                    Constant::None => {
                        todo!("{}", self.none_cant_be_a_type_error());
                    }
                    _ => {
                        todo!()
                    }
                },
                _ => {
                    panic!("{}", self.unsupported_type_error());
                }
            }
        }
    }
}

        } else {
            panic!();
        }
    }
}
