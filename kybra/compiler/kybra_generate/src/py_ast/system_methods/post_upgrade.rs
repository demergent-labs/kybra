use cdk_framework::{nodes::ActPostUpgradeMethod, CanisterMethodType};

use crate::{generators::canister_methods::post_upgrade, py_ast::PyAst};

impl PyAst<'_> {
    pub fn build_post_upgrade_method(&self) -> ActPostUpgradeMethod {
        let post_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => post_upgrade_function_def.build_params(),
            None => vec![],
        };

        let body = post_upgrade::generate_post_upgrade_method_body(
            post_upgrade_function_def_option,
            &self.entry_module_name,
        );

        ActPostUpgradeMethod { params, body }
    }
}
