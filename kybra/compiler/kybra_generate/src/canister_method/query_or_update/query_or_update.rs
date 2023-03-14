use cdk_framework::act::node::{
    canister_method::{CanisterMethodType, QueryOrUpdateDefinition},
    ReturnType,
};
use rustpython_parser::ast::{Constant, ExprKind, KeywordData, Located, StmtKind};

use super::rust;
use crate::{
    errors::KybraResult, method_utils::params::InternalOrExternal, source_map::SourceMapped,
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
        let returns = match &self.node {
            StmtKind::FunctionDef { returns, .. } => returns,
            _ => return false,
        };

        match returns {
            Some(returns) => match &returns.node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => id == "Async",
                    _ => false,
                },
                _ => false,
            },
            None => false,
        }
    }

    pub fn get_guard_function_name(&self) -> Option<String> {
        match &self.node {
            StmtKind::FunctionDef { decorator_list, .. } => {
                get_guard_function_name_from_decorator_list(decorator_list)
            }
            _ => None,
        }
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_manual(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => {
                    if id == "manual" {
                        return true;
                    } else {
                        return SourceMapped::new(slice.as_ref(), self.source_map.clone())
                            .is_manual();
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_query_or_update_definition(&self) -> KybraResult<QueryOrUpdateDefinition> {
        if !self.is_canister_method_type(CanisterMethodType::Query)
            && !self.is_canister_method_type(CanisterMethodType::Update)
        {
            return Err(crate::errors::unreachable());
        }
        match &self.node {
            StmtKind::FunctionDef { name, .. } => Ok(QueryOrUpdateDefinition {
                body: rust::generate_body(self)?,
                params: self.build_params(InternalOrExternal::Internal)?,
                is_manual: self.is_manual(),
                name: name.clone(),
                return_type: ReturnType::new(self.build_return_type()?),
                is_async: self.is_async(),
                cdk_name: "kybra".to_string(),
                guard_function_name: self.get_guard_function_name(),
            }),
            _ => Err(crate::errors::unreachable()),
        }
    }
}

fn get_guard_function_name_from_keywords(keywords: &Vec<Located<KeywordData>>) -> Option<String> {
    for keyword in keywords {
        if let Some(arg) = &keyword.node.arg {
            if arg != "guard" {
                continue;
            }
            if let ExprKind::Constant { value, .. } = &keyword.node.value.node {
                if let Constant::Str(string) = value {
                    return Some(string.to_string());
                }
            }
        }
    }
    None
}

fn get_guard_function_name_from_decorator_list(
    decorator_list: &Vec<Located<ExprKind>>,
) -> Option<String> {
    for decorator in decorator_list {
        if let ExprKind::Call { keywords, .. } = &decorator.node {
            return get_guard_function_name_from_keywords(&keywords);
        }
    }
    None
}
