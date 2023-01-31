use rustpython_parser::ast::{Constant, ExprKind};

use cdk_framework::{
    act::node::data_type::{primitive::Primitive, DataType, TypeRef},
    ToDataType,
};

use super::KybraExpr;

impl ToDataType for KybraExpr<'_> {
    fn to_data_type(&self) -> DataType {
        match &self.located_expr.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => Primitive::Blob.to_data_type(),
                "empty" => Primitive::Empty.to_data_type(),
                "float64" => Primitive::Float64.to_data_type(),
                "float32" => Primitive::Float32.to_data_type(),
                "int" => Primitive::Int.to_data_type(),
                "int64" => Primitive::Int64.to_data_type(),
                "int32" => Primitive::Int32.to_data_type(),
                "int16" => Primitive::Int16.to_data_type(),
                "int8" => Primitive::Int8.to_data_type(),
                "nat" => Primitive::Nat.to_data_type(),
                "nat64" => Primitive::Nat64.to_data_type(),
                "nat32" => Primitive::Nat32.to_data_type(),
                "nat16" => Primitive::Nat16.to_data_type(),
                "nat8" => Primitive::Nat8.to_data_type(),
                "null" => Primitive::Null.to_data_type(),
                "Principal" => Primitive::Principal.to_data_type(),
                "bool" => Primitive::Bool.to_data_type(),
                "reserved" => Primitive::Reserved.to_data_type(),
                "str" => Primitive::String.to_data_type(),
                "text" => Primitive::String.to_data_type(),
                "void" => Primitive::Void.to_data_type(),
                _ => DataType::TypeRef(TypeRef {
                    name: id.to_string(),
                }),
            },
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    "Async" => self.to_async(),
                    "opt" => self.to_opt(),
                    "list" => self.to_array(),
                    "tuple" => self.to_tuple(None),
                    "manual" => KybraExpr {
                        located_expr: slice,
                        source_map: self.source_map,
                    }
                    .to_data_type(),
                    _ => panic!("{}", self.invalid_subscript_value_error()),
                },
                _ => panic!("{}", self.invalid_subscript_value_error()),
            },
            // ExprKind::Subscript { value, slice, ctx } => todo!(),
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(string) => DataType::TypeRef(TypeRef {
                    name: string.clone(),
                }),
                Constant::None => {
                    todo!("{}", self.none_cant_be_a_type_error());
                }
                _ => {
                    todo!()
                }
            },
            ExprKind::BoolOp { .. } => todo!(),
            ExprKind::NamedExpr { .. } => todo!(),
            ExprKind::BinOp { .. } => todo!(),
            ExprKind::UnaryOp { .. } => todo!(),
            ExprKind::Lambda { .. } => todo!(),
            ExprKind::IfExp { .. } => todo!(),
            ExprKind::Dict { .. } => todo!(),
            ExprKind::Set { .. } => todo!(),
            ExprKind::ListComp { .. } => todo!(),
            ExprKind::SetComp { .. } => todo!(),
            ExprKind::DictComp { .. } => todo!(),
            ExprKind::GeneratorExp { .. } => todo!(),
            ExprKind::Await { .. } => todo!(),
            ExprKind::Yield { .. } => todo!(),
            ExprKind::YieldFrom { .. } => todo!(),
            ExprKind::Compare { .. } => todo!(),
            ExprKind::Call { .. } => todo!(),
            ExprKind::FormattedValue { .. } => todo!(),
            ExprKind::JoinedStr { .. } => todo!(),
            ExprKind::Attribute { .. } => todo!(),
            ExprKind::Starred { .. } => todo!(),
            ExprKind::List { .. } => todo!(),
            ExprKind::Tuple { .. } => {
                todo!("I don't think we can handle all of the types at once here. But what if we have tuples inside of tuples? What do we do then?")
            }
            ExprKind::Slice { .. } => todo!(),
        }
    }
}
