use cdk_framework::act::node::CandidType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

impl SourceMapped<&Located<StmtKind>> {
    pub fn build_return_type(&self) -> KybraResult<CandidType> {
        let returns = match &self.node {
            StmtKind::FunctionDef { returns, .. } => returns,
            _ => return Err(crate::errors::unreachable()),
        };

        match returns {
            Some(return_type) => Ok(SourceMapped::new(
                return_type.as_ref(),
                self.source_map.clone(),
            )
            .to_candid_type()?),
            None => Err(self.return_type_annotation_required_error()),
        }
    }
}
