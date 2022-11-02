use rustpython_parser::ast::ExprKind;

use cdk_framework::{ActDataType, ToActDataType};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_async(&self, alias_name: &Option<&String>) -> ActDataType {
        match &self.located_expr.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "Async" {
                            panic!("{}", "Async did not work")
                        }
                    }
                    _ => panic!("{}", "Async did not work"),
                }
                let kybra_expr = KybraExpr {
                    located_expr: slice,
                    source_map: self.source_map,
                };
                kybra_expr.to_act_data_type(alias_name)
            }
            _ => panic!("{}", "Async did not work"),
        }
    }
}
