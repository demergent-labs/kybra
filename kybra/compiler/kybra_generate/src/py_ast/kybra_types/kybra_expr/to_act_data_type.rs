use rustpython_parser::ast::ExprKind;

use crate::cdk_act::{nodes::data_type_nodes::ActPrimitiveLit, ActDataType, ToActDataType};

use super::KybraExpr;

impl ToActDataType for KybraExpr<'_> {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> ActDataType {
        match &self.located_expr.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "empty" => ActPrimitiveLit::Empty,
                "float64" => ActPrimitiveLit::Float64,
                "float32" => ActPrimitiveLit::Float32,
                "int" => ActPrimitiveLit::Int,
                "int64" => ActPrimitiveLit::Int64,
                "int32" => ActPrimitiveLit::Int32,
                "int16" => ActPrimitiveLit::Int16,
                "int8" => ActPrimitiveLit::Int8,
                "nat" => ActPrimitiveLit::Nat,
                "nat64" => ActPrimitiveLit::Nat64,
                "nat32" => ActPrimitiveLit::Nat32,
                "nat16" => ActPrimitiveLit::Nat16,
                "nat8" => ActPrimitiveLit::Nat8,
                "null" => ActPrimitiveLit::Null,
                "Principal" => ActPrimitiveLit::Principal,
                "bool" => ActPrimitiveLit::Bool,
                "reserved" => ActPrimitiveLit::Reserved,
                "str" => ActPrimitiveLit::String,
                "text" => ActPrimitiveLit::String,
                _ => panic!("{}", self.type_not_supported_error()),
            }
            .to_act_data_type(alias_name),
            _ => todo!(),
        }
    }
}
