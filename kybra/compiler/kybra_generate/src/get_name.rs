use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    candid_type::errors::{InvalidName, NotExactlyOneTarget},
    source_map::SourceMapped,
    Error,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn get_name(&self) -> Result<Option<String>, Error> {
        Ok(match &self.node {
            StmtKind::FunctionDef { name, .. } => Some(name.clone()),
            StmtKind::AsyncFunctionDef { name, .. } => Some(name.clone()),
            StmtKind::ClassDef { name, .. } => Some(name.clone()),
            StmtKind::Assign { targets, .. } => {
                if targets.len() != 1 {
                    return Err(NotExactlyOneTarget::err_from_stmt(self).into());
                }
                SourceMapped::new(&targets[0], self.source_map.clone()).get_name()
            }
            StmtKind::AnnAssign { target, .. } => {
                SourceMapped::new(target.as_ref(), self.source_map.clone()).get_name()
            }
            _ => None,
        })
    }

    pub fn get_name_or_err(&self) -> Result<String, Error> {
        match self.get_name()? {
            Some(name) => Ok(name),
            None => Err(InvalidName::err_from_stmt(self).into()),
        }
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn get_name(&self) -> Option<String> {
        match &self.node {
            ExprKind::Name { id, .. } => Some(id.clone()),
            _ => None,
        }
    }
}
