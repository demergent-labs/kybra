use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    act::node::data_type::{self, option, DataType, LiteralOrTypeAlias},
    ToActDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_opt(&self, alias_name: &Option<&String>) -> DataType {
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
                    source_map: self.source_map,
                };
                match alias_name {
                    Some(alias_name) => DataType::Option(data_type::Option {
                        act_type: LiteralOrTypeAlias::TypeAlias(option::TypeAlias {
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                            name: alias_name.clone().clone(),
                        }),
                    }),
                    None => DataType::Option(data_type::Option {
                        act_type: LiteralOrTypeAlias::Literal(option::Literal {
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                        }),
                    }),
                }
            }
            _ => panic!("{}", self.not_opt_error()),
        }
    }
}
