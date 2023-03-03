pub mod error;

use cdk_framework::act::node::data_type::Array;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_array(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "list",
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
                        if id != "list" {
                            panic!("Unreachable");
                        }
                    }
                    _ => panic!("Unreachable"),
                }
                let kybra_expr = SourceMapped::new(slice.as_ref(), self.source_map.clone());
                Ok(Array {
                    enclosed_type: Box::from(kybra_expr.to_data_type()?),
                })
            }
            _ => panic!("Unreachable"),
        }
    }
}
