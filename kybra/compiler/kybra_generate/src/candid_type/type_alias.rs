use cdk_framework::act::node::candid;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};

use super::errors::{InvalidTarget, NotExactlyOneTarget};

struct TypeAlias<'a> {
    target: &'a Located<ExprKind>,
    enclosed_expr: &'a Located<ExprKind>,
}

impl PyAst {
    pub fn build_type_aliases(&self) -> Result<Vec<candid::TypeAlias>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_type_alias())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn get_type_alias(&self) -> Result<Option<TypeAlias>, Error> {
        if let StmtKind::Assign { value, .. }
        | StmtKind::AnnAssign {
            value: Some(value), ..
        } = &self.node
        {
            if let ExprKind::Subscript { value, slice, .. } = &value.node {
                if let ExprKind::Name { id, .. } = &value.node {
                    match id == "Alias" {
                        true => {
                            let assign_target = match &self.node {
                                StmtKind::Assign { targets, .. } => {
                                    if targets.len() != 1 {
                                        return Err(NotExactlyOneTarget::err_from_stmt(self).into());
                                    }
                                    &targets[0]
                                }
                                StmtKind::AnnAssign { target, .. } => target.as_ref(),
                                _ => return Ok(None),
                            };
                            return Ok(Some(TypeAlias {
                                target: assign_target,
                                enclosed_expr: slice,
                            }));
                        }
                        false => return Ok(None),
                    }
                }
            }
        }
        Ok(None)
    }

    fn as_type_alias(&self) -> Result<Option<candid::TypeAlias>, Vec<Error>> {
        let type_alias = match self.get_type_alias().map_err(Into::<Vec<Error>>::into)? {
            Some(type_alias) => type_alias,
            None => return Ok(None),
        };

        let alias_name = match &type_alias.target.node {
            ExprKind::Name { id, .. } => id.clone(),
            _ => return Err(InvalidTarget::err_from_stmt(self).into()),
        };

        let enclosed_type = SourceMapped::new(type_alias.enclosed_expr, self.source_map.clone())
            .to_candid_type()?;
        Ok(Some(candid::TypeAlias {
            name: alias_name,
            aliased_type: Box::new(enclosed_type),
            type_params: vec![].into(),
        }))
    }
}
