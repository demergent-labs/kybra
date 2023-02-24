use cdk_framework::act::node::canister_method::{CanisterMethodType, PostUpgradeMethod};

use crate::{generators::canister_methods::post_upgrade, py_ast::PyAst};

impl PyAst {
    pub fn build_post_upgrade_method(&self) -> PostUpgradeMethod {
        let post_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => post_upgrade_function_def.build_params(),
            None => vec![],
        };

        let body = post_upgrade::generate_post_upgrade_method_body_cdk_refactor_name(
            post_upgrade_function_def_option,
            &self.entry_module_name,
        );

        PostUpgradeMethod { params, body }
    }
}