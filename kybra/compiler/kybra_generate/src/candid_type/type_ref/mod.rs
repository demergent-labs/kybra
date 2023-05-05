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
        let name = match &self.node {
            ExprKind::Name { id, .. } => id,
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(string) => string,
                _ => return Err(Unreachable::new_err()),
            },
            _ => return Err(Unreachable::new_err()),
        }
        .to_string();
        Ok(Some(TypeRef {
            name,
            type_arguments: vec![],
        }))
    }
}
