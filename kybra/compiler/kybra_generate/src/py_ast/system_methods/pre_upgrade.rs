use cdk_framework::{nodes::ActPreUpgradeMethod, CanisterMethodType};

use crate::{generators::canister_methods::pre_upgrade, py_ast::PyAst};

impl PyAst<'_> {
    pub fn build_pre_upgrade_method(&self) -> ActPreUpgradeMethod {
        let pre_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PreUpgrade);

        if pre_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let pre_upgrade_function_def_option = pre_upgrade_function_defs.get(0);

        let body = pre_upgrade::generate_pre_upgrade_method_body(pre_upgrade_function_def_option);

        ActPreUpgradeMethod { body }
    }
}
