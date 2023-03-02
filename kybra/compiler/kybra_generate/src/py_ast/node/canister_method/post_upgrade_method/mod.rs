pub mod errors;

use cdk_framework::act::node::canister_method::{CanisterMethodType, PostUpgradeMethod};

use crate::{errors::KybraResult, generators::canister_methods::post_upgrade, py_ast::PyAst};

impl PyAst {
    pub fn build_post_upgrade_method(&self) -> KybraResult<PostUpgradeMethod> {
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

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => post_upgrade_function_def.build_params(),
            None => vec![],
        };

        let body =
            post_upgrade::generate(post_upgrade_function_def_option, &self.entry_module_name);

        Ok(PostUpgradeMethod { params, body })
    }
}
