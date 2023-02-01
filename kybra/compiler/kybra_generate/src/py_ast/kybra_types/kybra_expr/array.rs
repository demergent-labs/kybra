use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    act::node::data_type::{array, Array, DataType, LiteralOrTypeAlias},
    ToActDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_array(&self, alias_name: &Option<&String>) -> DataType {
        match &self.located_expr.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "list" {
                            panic!("{}", self.not_array_error())
                        }
                    }
                    _ => panic!("{}", self.not_array_error()),
                }
                let kybra_expr = KybraExpr {
                    located_expr: slice,
                    source_map: self.source_map,
                };
                match alias_name {
                    Some(alias_name) => DataType::Array(Array {
                        act_type: LiteralOrTypeAlias::TypeAlias(array::TypeAlias {
                            name: alias_name.clone().clone(),
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                        }),
                    }),
                    None => DataType::Array(Array {
                        act_type: LiteralOrTypeAlias::Literal(array::Literal {
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                        }),
                    }),
                }
            }
            _ => panic!("{}", self.not_array_error()),
        }
    }
}
