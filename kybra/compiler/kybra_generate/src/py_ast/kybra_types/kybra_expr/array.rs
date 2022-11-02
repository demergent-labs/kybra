use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    nodes::data_type_nodes::{ActArray, ActArrayLiteral, ActArrayTypeAlias, LiteralOrTypeAlias},
    ActDataType, ToActDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_array(&self, alias_name: &Option<&String>) -> ActDataType {
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
                    Some(alias_name) => ActDataType::Array(ActArray {
                        act_type: LiteralOrTypeAlias::TypeAlias(ActArrayTypeAlias {
                            name: alias_name.clone().clone(),
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                        }),
                    }),
                    None => ActDataType::Array(ActArray {
                        act_type: LiteralOrTypeAlias::Literal(ActArrayLiteral {
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                        }),
                    }),
                }
            }
            _ => panic!("{}", self.not_array_error()),
        }
    }
}
