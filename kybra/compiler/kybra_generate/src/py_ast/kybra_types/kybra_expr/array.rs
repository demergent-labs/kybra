use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    act::node::data_type::{Array, DataType},
    ToDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_array(&self) -> DataType {
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
                    source_map: self.source_map.clone(),
                };
                DataType::Array(Array {
                    enclosed_type: Box::from(kybra_expr.to_data_type()),
                })
            }
            _ => panic!("{}", self.not_array_error()),
        }
    }
}
