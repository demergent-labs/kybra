use cdk_framework::act::node::data_type::TypeRef;
use rustpython_parser::ast::{Constant, ExprKind, Located};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

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

    pub fn to_type_ref(&self) -> Result<TypeRef, Message> {
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
                _ => Err(self.not_type_ref_error()),
            },
            _ => Err(self.not_type_ref_error()),
        }
    }

    pub fn not_type_ref_error(&self) -> Message {
        self.create_error_message("This is is not a type ref", "", None)
    }
}
