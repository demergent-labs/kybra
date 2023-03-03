pub mod errors;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
mod query_or_update;

use std::ops::Deref;

use cdk_framework::act::node::canister_method::CanisterMethodType;
use cdk_framework::act::node::param::Param;
use proc_macro2::TokenStream;
use rustpython_parser::ast::ExprKind;
use rustpython_parser::ast::Located;
use rustpython_parser::ast::Mod;
use rustpython_parser::ast::StmtKind;

use crate::errors::KybraResult;
use crate::generators::canister_methods;
use crate::py_ast::PyAst;
use crate::source_map::SourceMapped;

pub use query_or_update::query_method;
pub use query_or_update::update_method;

impl PyAst {
    fn get_canister_stmt_of_type(
        &self,
        method_type: CanisterMethodType,
    ) -> Vec<SourceMapped<&Located<StmtKind>>> {
        self.source_mapped_mods
            .iter()
            .fold(vec![], |mut acc, program| {
                acc.extend(match &program.deref() {
                    Mod::Module { body, .. } => body
                        .iter()
                        .filter(|stmt_kind| {
                            SourceMapped::new(*stmt_kind, program.source_map.clone())
                                .is_canister_method_type(method_type.clone())
                        })
                        .map(|stmt_kind| SourceMapped::new(stmt_kind, program.source_map.clone()))
                        .collect(),
                    _ => vec![],
                });
                acc
            })
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_canister_method_type(&self, canister_method_type: CanisterMethodType) -> bool {
        self.is_decorator_same_as(match canister_method_type {
            CanisterMethodType::Heartbeat => "heartbeat",
            CanisterMethodType::Init => "init",
            CanisterMethodType::InspectMessage => "inspect_message",
            CanisterMethodType::PostUpgrade => "post_upgrade",
            CanisterMethodType::PreUpgrade => "pre_upgrade",
            CanisterMethodType::Query => "query",
            CanisterMethodType::Update => "update",
        })
    }

    fn is_decorator_same_as(&self, decorator_name: &str) -> bool {
        match &self.node {
            StmtKind::FunctionDef { decorator_list, .. } => {
                decorator_list
                    .iter()
                    .any(|expr_kind| match &expr_kind.node {
                        ExprKind::Name { id, .. } => id == decorator_name,
                        ExprKind::Call { func, .. } => match &func.node {
                            ExprKind::Name { id, .. } => id == decorator_name,
                            _ => false,
                        },
                        _ => false,
                    })
            }
            _ => false,
        }
    }

    pub fn build_params(&self) -> KybraResult<Vec<Param>> {
        crate::errors::collect_kybra_results(match &self.node {
            StmtKind::FunctionDef { args, .. } => args
                .args
                .iter()
                .map(|arg| SourceMapped::new(arg, self.source_map.clone()).to_param())
                .collect(),
            _ => panic!("{}", self.not_a_function_def_error()),
        })
    }

    pub fn get_function_name(&self) -> String {
        match &self.node {
            StmtKind::FunctionDef { name, .. } => name.clone(),
            _ => panic!("{}", self.not_a_function_def_error()),
        }
    }

    pub fn generate_call_to_py_function(&self) -> KybraResult<TokenStream> {
        canister_methods::generate_call_to_py_function(self)
    }
}
