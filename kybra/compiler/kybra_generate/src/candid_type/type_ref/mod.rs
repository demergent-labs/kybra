pub mod errors;

use cdk_framework::act::node::candid::TypeRef;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{errors::Unreachable, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    fn is_type_ref(&self) -> bool {
        match &self.node {
            ExprKind::Name { .. } => true,
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_type_ref(&self) -> Result<Option<TypeRef>, Error> {
        if !self.is_type_ref() {
            return Ok(None);
        }
        Ok(Some(match &self.node {
            ExprKind::Name { id, .. } => TypeRef {
                name: id.to_string(),
                type_arguments: vec![],
            },
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(string) => TypeRef {
                    name: string.clone(),
                    type_arguments: vec![],
                },
                _ => return Err(Unreachable::new_err()),
            },
            _ => return Err(Unreachable::new_err()),
        }))
    }
}
