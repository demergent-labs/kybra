pub mod errors;

use cdk_framework::act::node::candid::Array;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_array(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Vec",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn to_array(&self) -> KybraResult<Array> {
        if !self.is_array() {
            return Err(self.not_array_error());
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "Vec" {
                            return Err(crate::errors::unreachable());
                        }
                    }
                    _ => return Err(crate::errors::unreachable()),
                }
                let kybra_expr = SourceMapped::new(slice.as_ref(), self.source_map.clone());
                Ok(Array {
                    enclosed_type: Box::from(kybra_expr.to_candid_type()?),
                })
            }
            _ => Err(crate::errors::unreachable()),
        }
    }

    pub fn array_uses_type_ref(&self, name: &str) -> bool {
        self.is_array() && self.enclosed_type_uses_type_ref(name)
    }
}
