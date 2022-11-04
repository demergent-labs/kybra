use rustpython_parser::ast::ExprKind;

use cdk_framework::{
    nodes::data_type_nodes::{ActOption, ActOptionLiteral, ActOptionTypeAlias, LiteralOrTypeAlias},
    ActDataType, ToActDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn to_opt(&self, alias_name: &Option<&String>) -> ActDataType {
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
                    Some(alias_name) => ActDataType::Option(ActOption {
                        act_type: LiteralOrTypeAlias::TypeAlias(ActOptionTypeAlias {
                            enclosed_type: Box::from(
                                kybra_expr.to_act_data_type(&Some(&alias_name)),
                            ),
                            name: alias_name.clone().clone(),
                        }),
                    }),
                    None => ActDataType::Option(ActOption {
                        act_type: LiteralOrTypeAlias::Literal(ActOptionLiteral {
                            enclosed_type: Box::from(kybra_expr.to_act_data_type(&None)),
                        }),
                    }),
                }
            }
            _ => panic!("{}", self.not_opt_error()),
        }
    }
}
