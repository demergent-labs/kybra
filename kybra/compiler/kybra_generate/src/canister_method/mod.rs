pub mod errors;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
mod query_or_update;
mod rust;

use std::ops::Deref;

use cdk_framework::act::node::candid::Primitive;
use cdk_framework::act::node::canister_method::CanisterMethodType;
use cdk_framework::act::node::CandidType;
use rustpython_parser::ast::Constant;
use rustpython_parser::ast::ExprKind;
use rustpython_parser::ast::KeywordData;
use rustpython_parser::ast::Located;
use rustpython_parser::ast::Mod;
use rustpython_parser::ast::StmtKind;

use crate::errors::KybraResult;
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
        self.has_decorator(match canister_method_type {
            CanisterMethodType::Heartbeat => "heartbeat",
            CanisterMethodType::Init => "init",
            CanisterMethodType::InspectMessage => "inspect_message",
            CanisterMethodType::PostUpgrade => "post_upgrade",
            CanisterMethodType::PreUpgrade => "pre_upgrade",
            CanisterMethodType::Query => "query",
            CanisterMethodType::Update => "update",
        })
    }

    fn has_decorator(&self, decorator_name: &str) -> bool {
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

    pub fn get_function_name(&self) -> KybraResult<String> {
        match &self.node {
            StmtKind::FunctionDef { name, .. } => Ok(name.clone()),
            _ => Err(crate::errors::unreachable()),
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

pub fn is_void(candid_type: CandidType) -> bool {
    if let CandidType::Primitive(primitive) = candid_type {
        if let Primitive::Void = primitive {
            return true;
        };
    }
    return false;
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
