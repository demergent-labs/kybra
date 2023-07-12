use rustpython_parser::ast::{Constant, ExprKind, Located, StmtKind};

use crate::{
    candid_type::errors::{InvalidName, NotExactlyOneTarget},
    source_map::SourceMapped,
    Error,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn get_name(&self) -> Result<Option<&str>, Error> {
        Ok(match &self.node {
            StmtKind::FunctionDef { name, .. } => Some(name),
            StmtKind::AsyncFunctionDef { name, .. } => Some(name),
            StmtKind::ClassDef { name, .. } => Some(name),
            StmtKind::Assign { targets, .. } => {
                if targets.len() != 1 {
                    return Err(NotExactlyOneTarget::err_from_stmt(self).into());
                }
                targets[0].get_name()
            }
            StmtKind::AnnAssign { target, .. } => target.get_name(),
            _ => None,
        })
    }

    pub fn get_name_or_err(&self) -> Result<&str, Error> {
        match self.get_name()? {
            Some(name) => Ok(name),
            None => Err(InvalidName::err_from_stmt(self).into()),
        }
    }
}

pub trait HasName {
    fn get_name(&self) -> Option<&str>;
}

impl HasName for Located<ExprKind> {
    fn get_name(&self) -> Option<&str> {
        match &self.node {
            ExprKind::Name { id, .. } => Some(id),
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(string) => Some(string),
                _ => None,
            },
            _ => None,
        }
    }
}
