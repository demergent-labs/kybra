pub mod errors;

use cdk_framework::act::node::data_type::Primitive;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::Message, source_map::SourceMapped};

impl SourceMapped<'_, Located<ExprKind>> {
    pub fn is_primitive(&self) -> bool {
        match &self.node.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => true,
                "empty" => true,
                "float64" => true,
                "float32" => true,
                "int" => true,
                "int64" => true,
                "int32" => true,
                "int16" => true,
                "int8" => true,
                "nat" => true,
                "nat64" => true,
                "nat32" => true,
                "nat16" => true,
                "nat8" => true,
                "null" => true,
                "Principal" => true,
                "bool" => true,
                "reserved" => true,
                "str" => true,
                "text" => true,
                "void" => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_primitive(&self) -> Result<Primitive, Message> {
        match &self.node.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => Ok(Primitive::Blob),
                "empty" => Ok(Primitive::Empty),
                "float64" => Ok(Primitive::Float64),
                "float32" => Ok(Primitive::Float32),
                "int" => Ok(Primitive::Int),
                "int64" => Ok(Primitive::Int64),
                "int32" => Ok(Primitive::Int32),
                "int16" => Ok(Primitive::Int16),
                "int8" => Ok(Primitive::Int8),
                "nat" => Ok(Primitive::Nat),
                "nat64" => Ok(Primitive::Nat64),
                "nat32" => Ok(Primitive::Nat32),
                "nat16" => Ok(Primitive::Nat16),
                "nat8" => Ok(Primitive::Nat8),
                "null" => Ok(Primitive::Null),
                "Principal" => Ok(Primitive::Principal),
                "bool" => Ok(Primitive::Bool),
                "reserved" => Ok(Primitive::Reserved),
                "str" => Ok(Primitive::String),
                "text" => Ok(Primitive::String),
                "void" => Ok(Primitive::Void),
                _ => Err(self.not_a_primitive_error()),
            },
            _ => Err(self.not_a_primitive_error()),
        }
    }
}
