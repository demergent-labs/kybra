pub mod errors;

use cdk_framework::{
    act::node::data_type::{tuple::Member, Tuple},
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<'_, Located<ExprKind>> {
    pub fn is_tuple(&self) -> bool {
        match &self.node.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "tuple",
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn to_tuple(&self, tuple_name: Option<String>) -> Result<Tuple, Message> {
        match &self.node.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "tuple" {
                            return Err(self.not_tuple_error());
                        }
                    }
                    _ => return Err(self.not_tuple_error()),
                }
                let kybra_elem_exprs = match &slice.node {
                    ExprKind::Tuple { elts, .. } => elts
                        .iter()
                        .map(|elt| SourceMapped {
                            node: elt,
                            source_map: self.source_map.clone(),
                        })
                        .collect(),
                    _ => {
                        vec![SourceMapped {
                            node: slice.as_ref(),
                            source_map: self.source_map.clone(),
                        }]
                    }
                };
                let act_elems = kybra_elem_exprs
                    .iter()
                    .map(|kybra_elem| Member {
                        type_: kybra_elem.to_data_type(),
                    })
                    .collect();
                Ok(Tuple {
                    name: tuple_name,
                    members: act_elems,
                })
            }
            _ => Err(self.not_tuple_error()),
        }
    }

    pub fn not_tuple_error(&self) -> Message {
        self.create_error_message("This is is not a tuple", "", None)
    }
}

impl SourceMapped<'_, Located<StmtKind>> {
    pub fn is_tuple(&self) -> bool {
        match &self.node.node {
            StmtKind::Assign { value, .. } => SourceMapped {
                node: value.as_ref(),
                source_map: self.source_map.clone(),
            }
            .is_tuple(),
            _ => false,
        }
    }

    pub fn as_tuple(&self) -> Option<Tuple> {
        self.to_tuple().ok()
    }

    pub fn to_tuple(&self) -> Result<Tuple, Message> {
        match &self.node.node {
            StmtKind::Assign { targets, value, .. } => {
                if targets.len() > 1 {
                    return Err(self.multiple_targets_error());
                }
                let tuple_name = match &targets[0].node {
                    ExprKind::Name { id, .. } => id,
                    _ => return Err(self.invalid_target_error()),
                };
                SourceMapped {
                    node: value.as_ref(),
                    source_map: self.source_map.clone(),
                }
                .to_tuple(Some(tuple_name.clone()))
            }
            _ => panic!("{}", self.not_a_tuple_error()),
        }
    }
}
