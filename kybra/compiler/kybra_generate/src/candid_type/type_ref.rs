use cdk_framework::act::node::candid::TypeRef;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::source_map::SourceMapped;

impl SourceMapped<&Located<ExprKind>> {
    fn get_type_ref_name(&self) -> Option<String> {
        match &self.node {
            ExprKind::Name { id, .. } => Some(id.clone()),
            ExprKind::Constant { value, .. } => match value {
                Constant::Str(str) => Some(str.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_type_ref(&self) -> Option<TypeRef> {
        let name = match self.get_type_ref_name() {
            Some(type_ref) => type_ref,
            None => return None,
        };
        Some(TypeRef {
            name,
            type_arguments: vec![],
        })
    }
}
