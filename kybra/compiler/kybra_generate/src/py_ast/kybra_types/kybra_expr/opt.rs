use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    act::node::data_type::{self, DataType, TypeAlias},
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
                    programs: self.programs,
                    source_map: self.source_map,
                };
                let act_option = DataType::Option(data_type::Option {
                    enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                });
                match alias_name {
                    Some(alias_name) => DataType::TypeAlias(TypeAlias {
                        name: alias_name.clone().clone(),
                        aliased_type: Box::from(act_option),
                    }),
                    None => act_option,
                }
            }
            _ => panic!("{}", self.not_opt_error()),
        }
    }
}
