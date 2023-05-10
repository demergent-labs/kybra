pub mod tuple_members;

use cdk_framework::act::node::candid::Tuple;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    constants::TUPLE, errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error,
};

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
    pub fn as_tuple(&self, tuple_name: Option<String>) -> Result<Option<Tuple>, Vec<Error>> {
        match self.get_subscript_slice_for(TUPLE) {
            Some(slice) => {
                let tuple_members_exprs = match &slice.node {
                    ExprKind::Tuple { elts, .. } => elts
                        .iter()
                        .map(|elt| SourceMapped::new(elt, self.source_map.clone()))
                        .collect(),
                    _ => {
                        vec![SourceMapped::new(slice, self.source_map.clone())]
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
            None => Ok(None),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn as_tuple(&self) -> Result<Option<Tuple>, Vec<Error>> {
        match &self.node {
            StmtKind::Assign { value, .. } => {
                let name = self.get_name_or_err()?;
                Ok(SourceMapped::new(value.as_ref(), self.source_map.clone())
                    .as_tuple(Some(name.to_string()))?)
            }
            _ => Ok(None),
        }
    }
}
