use cdk_framework::act::node::canister_method::{CanisterMethodType, PreUpgradeMethod};

use crate::{generators::canister_methods::pre_upgrade, py_ast::PyAst};

impl PyAst {
    pub fn build_pre_upgrade_method(&self) -> Option<PreUpgradeMethod> {
        let pre_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PreUpgrade);

        if pre_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let pre_upgrade_function_def_option = pre_upgrade_function_defs.get(0);

        let body = pre_upgrade::generate(pre_upgrade_function_def_option);

        Some(PreUpgradeMethod { body })
    }
}
