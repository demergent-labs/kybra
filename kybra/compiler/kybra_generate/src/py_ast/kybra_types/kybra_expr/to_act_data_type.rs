use rustpython_parser::ast::{Constant, ExprKind};

use cdk_framework::{
    act::node::data_type::{primitive::Primitive, DataType, TypeRef},
    ToActDataType,
};

use super::KybraExpr;

impl ToActDataType for KybraExpr<'_> {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> DataType {
        match &self.located_expr.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => Primitive::Blob.to_act_data_type(alias_name),
                "empty" => Primitive::Empty.to_act_data_type(alias_name),
                "float64" => Primitive::Float64.to_act_data_type(alias_name),
                "float32" => Primitive::Float32.to_act_data_type(alias_name),
                "int" => Primitive::Int.to_act_data_type(alias_name),
                "int64" => Primitive::Int64.to_act_data_type(alias_name),
                "int32" => Primitive::Int32.to_act_data_type(alias_name),
                "int16" => Primitive::Int16.to_act_data_type(alias_name),
                "int8" => Primitive::Int8.to_act_data_type(alias_name),
                "nat" => Primitive::Nat.to_act_data_type(alias_name),
                "nat64" => Primitive::Nat64.to_act_data_type(alias_name),
                "nat32" => Primitive::Nat32.to_act_data_type(alias_name),
                "nat16" => Primitive::Nat16.to_act_data_type(alias_name),
                "nat8" => Primitive::Nat8.to_act_data_type(alias_name),
                "null" => Primitive::Null.to_act_data_type(alias_name),
                "Principal" => Primitive::Principal.to_act_data_type(alias_name),
                "bool" => Primitive::Bool.to_act_data_type(alias_name),
                "reserved" => Primitive::Reserved.to_act_data_type(alias_name),
                "str" => Primitive::String.to_act_data_type(alias_name),
                "text" => Primitive::String.to_act_data_type(alias_name),
                "void" => Primitive::Void.to_act_data_type(alias_name),
                _ => DataType::TypeRef(TypeRef {
                    name: id.to_string(),
                }),
            },
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    "Async" => self.to_async(alias_name),
                    "opt" => self.to_opt(alias_name),
                    "list" => self.to_array(alias_name),
                    "tuple" => self.to_tuple(alias_name),
                    "manual" => KybraExpr {
                        located_expr: slice,
                        source_map: self.source_map,
                    }
                    .to_act_data_type(alias_name),
                    _ => panic!("{}", self.invalid_subscript_value_error()),
                },
                _ => panic!("{}", self.invalid_subscript_value_error()),
            },
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
