pub mod errors;

use cdk_framework::act::node::data_type::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_opt(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "opt",
                _ => false,
            },
            _ => false,
        }
    }

    pub(super) fn to_opt(&self) -> KybraResult<Opt> {
        if !self.is_opt() {
            return Err(self.not_opt_error());
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "opt" {
                            return Err(crate::errors::unreachable());
                        }
                    }
                    _ => return Err(crate::errors::unreachable()),
                }
                let kybra_expr = SourceMapped::new(slice.as_ref(), self.source_map.clone());
                Ok(Opt {
                    enclosed_type: Box::from(kybra_expr.to_data_type()?),
                })
            }
            _ => return Err(crate::errors::unreachable()),
        }
    }
}
