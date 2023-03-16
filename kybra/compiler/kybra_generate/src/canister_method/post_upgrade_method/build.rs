use cdk_framework::act::node::canister_method::{CanisterMethodType, PostUpgradeMethod};

use super::rust;
use crate::{
    canister_method, errors::KybraResult, method_utils::params::InternalOrExternal, py_ast::PyAst,
};

impl PyAst {
    pub fn build_post_upgrade_method(&self) -> KybraResult<Option<PostUpgradeMethod>> {
        let post_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            return Err(post_upgrade_function_defs
                .iter()
                .map(|post_upgrade_function_def| {
                    post_upgrade_function_def.only_one_post_upgrade_method_allowed_error()
                })
                .collect());
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        if let Some(post_upgrade_function_def) = post_upgrade_function_def_option {
            if !canister_method::is_void(post_upgrade_function_def.build_return_type()?) {
                return Err(post_upgrade_function_def.post_upgrade_method_must_return_void_error());
            }
        }

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => {
                post_upgrade_function_def.build_params(InternalOrExternal::Internal)?
            }
            None => vec![],
        };

        let body = rust::generate(post_upgrade_function_def_option, &self.entry_module_name)?;

        Ok(Some(PostUpgradeMethod {
            params,
            body,
            guard_function_name: None,
        }))
    }
}
