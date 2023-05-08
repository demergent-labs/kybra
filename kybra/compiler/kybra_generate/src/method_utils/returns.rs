use cdk_framework::act::node::CandidType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::Unreachable, source_map::SourceMapped, Error};

use super::errors::ReturnTypeAnnotationRequired;

impl SourceMapped<&Located<StmtKind>> {
    pub fn build_return_type(&self) -> Result<CandidType, Vec<Error>> {
        let returns = match &self.node {
            StmtKind::FunctionDef { returns, .. } => returns,
            _ => return Err(Unreachable::error().into()),
        };

        match returns {
            Some(return_type) => Ok(SourceMapped::new(
                return_type.as_ref(),
                self.source_map.clone(),
            )
            .to_candid_type()?),
            None => Err(ReturnTypeAnnotationRequired::err_from_stmt(self).into()),
        }
    }
}
