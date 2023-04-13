pub mod errors;

use cdk_framework::act::node::candid::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped, Error};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_opt(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "Opt",
                _ => false,
            },
            _ => false,
        }
    }

    // TODO find a neutral place for this to live
    pub fn get_enclosed_type(&self) -> KybraResult<SourceMapped<&Located<ExprKind>>> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => {
                Ok(SourceMapped::new(slice.as_ref(), self.source_map.clone()))
            }
            _ => return Err(crate::errors::unreachable()),
        }
    }

    pub(super) fn to_opt(&self) -> Result<Opt, Vec<Error>> {
        if !self.is_opt() {
            return Err(vec![self.not_opt_error()]);
        }
        Ok(Opt {
            enclosed_type: Box::from(
                match self.get_enclosed_type() {
                    Ok(enclosed_type) => enclosed_type,
                    Err(err) => return Err(vec![err]),
                }
                .to_candid_type()?,
            ),
        })
    }
}
