use cdk_framework::{
    act::node::{
        canister_method::{CanisterMethodType, QueryOrUpdateDefinition},
        ReturnType,
    },
    traits::CollectResults,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use super::rust;
use crate::{
    constants::{ASYNC, MANUAL},
    get_name::HasName,
    method_utils::params::InternalOrExternal,
    source_map::SourceMapped,
    Error,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_manual(&self) -> bool {
        match &self.node {
            StmtKind::FunctionDef { returns, .. } => match returns {
                Some(returns) => {
                    SourceMapped::new(returns.as_ref(), self.source_map.clone()).is_manual()
                }
                None => false,
            },
            _ => false,
        }
    }

    pub fn is_async(&self) -> bool {
        if let StmtKind::FunctionDef {
            returns: Some(returns),
            ..
        } = &self.node
        {
            if let ExprKind::Subscript { value, .. } = &returns.node {
                return value.get_name() == Some(ASYNC);
            }
        }
        false
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_manual(&self) -> bool {
        if let ExprKind::Subscript { value, slice, .. } = &self.node {
            if let Some(MANUAL) = &value.get_name() {
                return true;
            }
            return SourceMapped::new(slice.as_ref(), self.source_map.clone()).is_manual();
        }
        false
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_query_or_update_definition(
        &self,
    ) -> Result<Option<QueryOrUpdateDefinition>, Vec<Error>> {
        if !self.is_canister_method_type(CanisterMethodType::Query)
            && !self.is_canister_method_type(CanisterMethodType::Update)
        {
            return Ok(None);
        }
        let (body, params, return_type, guard_function_name) = (
            rust::generate_body(self),
            self.build_params(InternalOrExternal::Internal),
            self.build_return_type(),
            self.get_guard_function_name().map_err(Error::into),
        )
            .collect_results()?;
        match &self.node {
            StmtKind::FunctionDef { name, .. } => Ok(Some(QueryOrUpdateDefinition {
                body,
                params,
                is_manual: self.is_manual(),
                name: name.clone(),
                return_type: ReturnType::new(return_type),
                is_async: self.is_async(),
                guard_function_name,
            })),
            _ => Ok(None),
        }
    }
}
