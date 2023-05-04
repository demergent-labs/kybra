use cdk_framework::act::node::candid::Primitive;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::Unreachable, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    fn is_primitive(&self) -> bool {
        match &self.node {
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

    pub fn as_primitive(&self) -> Result<Option<Primitive>, Error> {
        if !self.is_primitive() {
            return Ok(None);
        }
        Ok(Some(match &self.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => Primitive::Blob,
                "empty" => Primitive::Empty,
                "float64" => Primitive::Float64,
                "float32" => Primitive::Float32,
                "int" => Primitive::Int,
                "int64" => Primitive::Int64,
                "int32" => Primitive::Int32,
                "int16" => Primitive::Int16,
                "int8" => Primitive::Int8,
                "nat" => Primitive::Nat,
                "nat64" => Primitive::Nat64,
                "nat32" => Primitive::Nat32,
                "nat16" => Primitive::Nat16,
                "nat8" => Primitive::Nat8,
                "null" => Primitive::Null,
                "Principal" => Primitive::Principal,
                "bool" => Primitive::Bool,
                "reserved" => Primitive::Reserved,
                "str" => Primitive::String,
                "text" => Primitive::String,
                "void" => Primitive::Void,
                _ => return Err(Unreachable::new_err()),
            },
            _ => return Err(Unreachable::new_err()),
        }))
    }
}
