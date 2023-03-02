pub mod errors;

use cdk_framework::act::node::data_type::{tuple::Member, Tuple};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::Message, py_ast::PyAst, source_map::SourceMapped};

impl PyAst {
    pub fn build_tuples(&self) -> Vec<Tuple> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_tuple())
            .collect()
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_tuple(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "tuple",
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn to_tuple(&self, tuple_name: Option<String>) -> Result<Tuple, Message> {
        match &self.node {
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
                        .map(|elt| SourceMapped::new(elt, self.source_map.clone()))
                        .collect(),
                    _ => {
                        vec![SourceMapped::new(slice.as_ref(), self.source_map.clone())]
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
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_tuple(&self) -> bool {
        match &self.node {
            StmtKind::Assign { value, .. } => {
                SourceMapped::new(value.as_ref(), self.source_map.clone()).is_tuple()
            }
            _ => false,
        }
    }

    pub fn as_tuple(&self) -> Option<Tuple> {
        self.to_tuple().ok()
    }

    pub fn to_tuple(&self) -> Result<Tuple, Message> {
        if !self.is_tuple() {
            return Err(self.not_a_tuple_error());
        }
        match &self.node {
            StmtKind::Assign { targets, value, .. } => {
                if targets.len() > 1 {
                    return Err(self.multiple_targets_error());
                }
                let tuple_name = match &targets[0].node {
                    ExprKind::Name { id, .. } => id,
                    _ => return Err(self.invalid_target_error()),
                };
                SourceMapped::new(value.as_ref(), self.source_map.clone())
                    .to_tuple(Some(tuple_name.clone()))
            }
            _ => panic!("{}", self.not_a_tuple_error()),
        }
    }
}
