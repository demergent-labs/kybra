// use rustpython_parser::ast::ExprKind;

// use cdk_framework::{
//     act::node::data_type::{tuple::Member, DataType, Tuple},
//     ToDataType,
// };

// use super::KybraExpr;

// impl KybraExpr<'_> {
//     pub fn is_tuple(&self) -> bool {
//         match &self.located_expr.node {
//             ExprKind::Subscript { value, .. } => match &value.node {
//                 ExprKind::Name { id, .. } => id == "tuple",
//                 _ => false,
//             },
//             _ => false,
//         }
//     }

//     pub fn to_tuple(&self, tuple_name: Option<&String>) -> DataType {
//         match &self.located_expr.node {
//             ExprKind::Subscript { value, slice, .. } => {
//                 match &value.node {
//                     ExprKind::Name { id, .. } => {
//                         if id != "tuple" {
//                             panic!("{}", self.not_tuple_error())
//                         }
//                     }
//                     _ => panic!("{}", self.not_tuple_error()),
//                 }
//                 let kybra_elem_exprs = match &slice.node {
//                     ExprKind::Tuple { elts, .. } => elts
//                         .iter()
//                         .map(|elt| KybraExpr {
//                             located_expr: elt,
//                             source_map: self.source_map.clone(),
//                         })
//                         .collect(),
//                     _ => {
//                         vec![KybraExpr {
//                             located_expr: slice,
//                             source_map: self.source_map.clone(),
//                         }]
//                     }
//                 };
//                 let act_elems = kybra_elem_exprs
//                     .iter()
//                     .map(|kybra_elem| Member {
//                         type_: kybra_elem.to_data_type(),
//                     })
//                     .collect();
//                 match tuple_name {
//                     Some(alias_name) => DataType::Tuple(Tuple {
//                         name: Some(alias_name.clone().clone()),
//                         members: act_elems,
//                     }),
//                     None => DataType::Tuple(Tuple {
//                         name: None,
//                         members: act_elems,
//                     }),
//                 }
//             }
//             _ => panic!("{}", self.not_tuple_error()),
//         }
//     }
// }
