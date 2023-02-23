use crate::{kybra_ast::NewPyAst, source_map::SourceMapped};
use cdk_framework::act::node::canister_method::{CanisterMethodType, QueryMethod};
use rustpython_parser::ast::{Located, StmtKind};

impl NewPyAst {
    pub fn build_query_methods(&self) -> Vec<QueryMethod> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_query_method())
            .collect()
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_query_method(&self) -> Option<QueryMethod> {
        if !self.is_canister_method_type(CanisterMethodType::Query) {
            return None;
        }
        match &self.node.node {
            StmtKind::FunctionDef { name, .. } => Some(QueryMethod {
                body: self.generate_body(),
                params: self.build_act_params(),
                is_manual: self.is_manual(),
                name: name.clone(),
                return_type: self.build_act_return_type(),
                is_async: self.is_async(),
                cdk_name: "kybra".to_string(),
                guard_function_name: self.get_guard_function_name(),
            }),
            _ => None,
        }
    }
}
