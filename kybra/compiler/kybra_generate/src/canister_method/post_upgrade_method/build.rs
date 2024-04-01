use super::rust;
use cdk_framework::{
    act::node::canister_method::{CanisterMethodType, PostUpgradeMethod},
    traits::CollectResults,
};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    canister_method::{
        self,
        errors::{MultipleSystemMethods, ReturnTypeMustBeVoid},
    },
    errors::CollectResults as OtherCollectResults,
    method_utils::params::InternalOrExternal,
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

impl PyAst {
    pub fn build_post_upgrade_method(&self) -> Result<PostUpgradeMethod, Vec<Error>> {
        let post_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PostUpgrade);

        let post_upgrade_methods =
            build_post_upgrade_methods(&post_upgrade_function_defs, &self.entry_module_name);

        if post_upgrade_function_defs.len() > 1 {
            return Err(vec![
                MultipleSystemMethods::err_from_stmt(
                    &post_upgrade_function_defs,
                    CanisterMethodType::PostUpgrade,
                )
                .into(),
                post_upgrade_methods.err().unwrap_or_default(),
            ]
            .concat());
        }

        Ok(match post_upgrade_methods?.pop() {
            Some(post_upgrade_method) => post_upgrade_method,
            None => build_post_upgrade_method(None, &self.entry_module_name)?,
        })
    }
}

fn build_post_upgrade_methods(
    post_upgrade_function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<Vec<PostUpgradeMethod>, Vec<Error>> {
    post_upgrade_function_defs
        .iter()
        .map(|post_upgrade_function_def| {
            build_post_upgrade_method(Some(post_upgrade_function_def), entry_module_name)
        })
        .collect_results()
}

fn build_post_upgrade_method(
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<PostUpgradeMethod, Vec<Error>> {
    match post_upgrade_function_def_option {
        Some(post_upgrade_function_def) => {
            let (params, return_type) = (
                post_upgrade_function_def.build_params(InternalOrExternal::Internal),
                post_upgrade_function_def.build_return_type(),
            )
                .collect_results()?;

            if !canister_method::is_void(return_type) {
                return Err(ReturnTypeMustBeVoid::err_from_stmt(
                    post_upgrade_function_def,
                    CanisterMethodType::PostUpgrade,
                )
                .into());
            }

            Ok(PostUpgradeMethod {
                params: params.clone(),
                body: rust::generate(post_upgrade_function_def_option, entry_module_name)?,
            })
        }
        None => Ok(PostUpgradeMethod {
            params: vec![],
            body: rust::generate(post_upgrade_function_def_option, entry_module_name)?,
        }),
    }
}
