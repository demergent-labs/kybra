use rustpython_parser::ast::ExprKind;

use crate::py_ast::traits::GenerateInlineName;
use cdk_framework::{
    nodes::data_type_nodes::{
        act_tuple::{Tuple, TupleLiteral, TupleTypeAlias},
        ActTuple, ActTupleElem, LiteralOrTypeAlias,
    },
    ActDataType, ToActDataType,
};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub fn is_tuple(&self) -> bool {
        match &self.located_expr.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "tuple",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_tuple(&self, alias_name: &Option<&String>) -> ActDataType {
        match &self.located_expr.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "tuple" {
                            panic!("{}", self.not_tuple_error())
                        }
                    }
                    _ => panic!("{}", self.not_tuple_error()),
                }
                let kybra_elem_exprs = match &slice.node {
                    ExprKind::Tuple { elts, .. } => elts
                        .iter()
                        .map(|elt| KybraExpr {
                            located_expr: elt,
                            source_map: self.source_map,
                        })
                        .collect(),
                    _ => {
                        vec![KybraExpr {
                            located_expr: slice,
                            source_map: self.source_map,
                        }]
                    }
                };
                let act_elems = kybra_elem_exprs
                    .iter()
                    .map(|kybra_elem| ActTupleElem {
                        elem_type: kybra_elem.to_act_data_type(&None),
                    })
                    .collect();
                match alias_name {
                    Some(_) => ActDataType::Tuple(ActTuple {
                        act_type: LiteralOrTypeAlias::TypeAlias(TupleTypeAlias {
                            tuple: Tuple {
                                name: alias_name.unwrap().clone(),
                                elems: act_elems,
                            },
                        }),
                    }),
                    None => ActDataType::Tuple(ActTuple {
                        act_type: LiteralOrTypeAlias::Literal(TupleLiteral {
                            tuple: Tuple {
                                name: self.generate_inline_name(),
                                elems: act_elems,
                            },
                        }),
                    }),
                }
            }
            _ => panic!("{}", self.not_tuple_error()),
        }
    }
}
