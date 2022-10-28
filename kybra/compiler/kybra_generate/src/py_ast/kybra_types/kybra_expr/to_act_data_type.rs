use rustpython_parser::ast::ExprKind;

use crate::cdk_act::{
    nodes::data_type_nodes::{ActPrimitiveLit, ActTypeRef, ActTypeRefLit, LiteralOrTypeAlias},
    ActDataType, ToActDataType,
};

use super::KybraExpr;

impl ToActDataType for KybraExpr<'_> {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> ActDataType {
        match &self.located_expr.node {
            ExprKind::Name { id, .. } => match &id[..] {
                "blob" => ActPrimitiveLit::Blob.to_act_data_type(alias_name),
                "empty" => ActPrimitiveLit::Empty.to_act_data_type(alias_name),
                "float64" => ActPrimitiveLit::Float64.to_act_data_type(alias_name),
                "float32" => ActPrimitiveLit::Float32.to_act_data_type(alias_name),
                "int" => ActPrimitiveLit::Int.to_act_data_type(alias_name),
                "int64" => ActPrimitiveLit::Int64.to_act_data_type(alias_name),
                "int32" => ActPrimitiveLit::Int32.to_act_data_type(alias_name),
                "int16" => ActPrimitiveLit::Int16.to_act_data_type(alias_name),
                "int8" => ActPrimitiveLit::Int8.to_act_data_type(alias_name),
                "nat" => ActPrimitiveLit::Nat.to_act_data_type(alias_name),
                "nat64" => ActPrimitiveLit::Nat64.to_act_data_type(alias_name),
                "nat32" => ActPrimitiveLit::Nat32.to_act_data_type(alias_name),
                "nat16" => ActPrimitiveLit::Nat16.to_act_data_type(alias_name),
                "nat8" => ActPrimitiveLit::Nat8.to_act_data_type(alias_name),
                "null" => ActPrimitiveLit::Null.to_act_data_type(alias_name),
                "Principal" => ActPrimitiveLit::Principal.to_act_data_type(alias_name),
                "bool" => ActPrimitiveLit::Bool.to_act_data_type(alias_name),
                "reserved" => ActPrimitiveLit::Reserved.to_act_data_type(alias_name),
                "str" => ActPrimitiveLit::String.to_act_data_type(alias_name),
                "text" => ActPrimitiveLit::String.to_act_data_type(alias_name),
                _ => ActDataType::TypeRef(ActTypeRef {
                    act_type: LiteralOrTypeAlias::Literal(ActTypeRefLit {
                        name: id.to_string(),
                    }),
                }),
            },
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    "opt" => self.to_opt(),
                    "list" => self.to_array(),
                    _ => panic!("{}", self.invalid_subscript_value_error()),
                },
                _ => panic!("{}", self.invalid_subscript_value_error()),
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
            ExprKind::Constant { .. } => todo!(),
            ExprKind::Attribute { .. } => todo!(),
            ExprKind::Starred { .. } => todo!(),
            ExprKind::List { .. } => todo!(),
            ExprKind::Tuple { .. } => todo!(),
            ExprKind::Slice { .. } => todo!(),
        }
    }
}
