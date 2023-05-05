pub mod tuple_members;

use cdk_framework::act::node::candid::Tuple;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    candid_type::errors::InvalidTarget,
    errors::{CollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

use super::errors::NotExactlyOneTarget;

impl PyAst {
    pub fn build_tuples(&self) -> Result<Vec<Tuple>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_tuple())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<ExprKind>> {
    fn is_tuple(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Tuple",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_tuple(&self, tuple_name: Option<String>) -> Result<Option<Tuple>, Vec<Error>> {
        if !self.is_tuple() {
            return Ok(None);
        }
        match &self.node {
            ExprKind::Subscript { slice, .. } => {
                let tuple_members_exprs = match &slice.node {
                    ExprKind::Tuple { elts, .. } => elts
                        .iter()
                        .map(|elt| SourceMapped::new(elt, self.source_map.clone()))
                        .collect(),
                    _ => {
                        vec![SourceMapped::new(slice.as_ref(), self.source_map.clone())]
                    }
                };
                let elems = tuple_members_exprs
                    .iter()
                    .map(|kybra_elem| kybra_elem.as_tuple_member())
                    .collect_results()?;
                Ok(Some(Tuple {
                    name: tuple_name,
                    elems,
                    type_params: vec![].into(),
                }))
            }
            _ => Err(Unreachable::new_err().into()),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn as_tuple(&self) -> Result<Option<Tuple>, Vec<Error>> {
        match &self.node {
            StmtKind::Assign { targets, value, .. } => {
                if targets.len() != 1 {
                    return Err(NotExactlyOneTarget::err_from_stmt(self).into());
                }
                let tuple_name = match &targets[0].node {
                    ExprKind::Name { id, .. } => id,
                    _ => return Err(InvalidTarget::err_from_stmt(self).into()),
                };
                Ok(SourceMapped::new(value.as_ref(), self.source_map.clone())
                    .as_tuple(Some(tuple_name.clone()))?)
            }
            _ => Ok(None),
        }
    }
}
