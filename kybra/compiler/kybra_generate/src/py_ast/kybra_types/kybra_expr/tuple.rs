use rustpython_parser::ast::ExprKind;

use crate::py_ast::traits::GenerateInlineName;
use cdk_framework::{
    act::node::data_type::{tuple::Elem, DataType, Tuple},
    ToActDataType,
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

    pub fn to_tuple(&self, alias_name: &Option<&String>) -> DataType {
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
                    .map(|kybra_elem| Elem {
                        elem_type: kybra_elem.to_act_data_type(&None),
                    })
                    .collect();
                match alias_name {
                    Some(alias_name) => DataType::Tuple(Tuple {
                        name: alias_name.clone().clone(),
                        elems: act_elems,
                    }),
                    None => DataType::Tuple(Tuple {
                        name: self.generate_inline_name(),
                        elems: act_elems,
                    }),
                }
            }
            _ => panic!("{}", self.not_tuple_error()),
        }
    }
}
