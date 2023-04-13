pub mod errors;

use cdk_framework::act::node::candid::Opt;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::KybraResult, source_map::SourceMapped};

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

    // TODO find a neutral place for this to live
    pub fn enclosed_type_uses_type_ref(&self, name: &str) -> bool {
        match &self.get_enclosed_type() {
            Ok(enclosed_type) => enclosed_type.uses_type_ref(name),
            Err(_) => false,
        }
    }

    pub(super) fn to_opt(&self) -> KybraResult<Opt> {
        if !self.is_opt() {
            return Err(self.not_opt_error());
        }
        Ok(Opt {
            enclosed_type: Box::from(self.get_enclosed_type()?.to_candid_type()?),
        })
    }

    pub fn opt_uses_type_ref(&self, name: &str) -> bool {
        self.is_opt() && self.enclosed_type_uses_type_ref(name)
    }
}
