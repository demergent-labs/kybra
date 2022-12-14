use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    act::node::data_type::{DataType, Opt},
    ToDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_opt(&self) -> DataType {
        match &self.located_expr.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "opt" {
                            panic!("{}", self.not_opt_error())
                        }
                    }
                    _ => panic!("{}", self.not_opt_error()),
                }
                let kybra_expr = KybraExpr {
                    located_expr: slice,
                    source_map: self.source_map.clone(),
                };
                DataType::Opt(Opt {
                    enclosed_type: Box::from(kybra_expr.to_data_type()),
                })
            }
            _ => panic!("{}", self.not_opt_error()),
        }
    }
}
