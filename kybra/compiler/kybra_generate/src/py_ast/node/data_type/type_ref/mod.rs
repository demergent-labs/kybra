pub mod errors;

use cdk_framework::act::node::data_type::TypeRef;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_type_ref(&self) -> bool {
        match &self.node {
            ExprKind::Name { .. } => true,
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_type_ref(&self) -> KybraResult<TypeRef> {
        if !self.is_type_ref() {
            return Err(self.not_type_ref_error());
        }
        match &self.node {
            ExprKind::Name { id, .. } => Ok(TypeRef {
                name: id.to_string(),
            }),
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(string) => Ok(TypeRef {
                    name: string.clone(),
                }),
                _ => panic!("Unreachable"),
            },
            _ => panic!("Unreachable"),
        }
    }
}
