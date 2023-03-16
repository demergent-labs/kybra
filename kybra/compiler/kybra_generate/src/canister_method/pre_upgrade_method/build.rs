use cdk_framework::act::node::canister_method::{CanisterMethodType, PreUpgradeMethod};

use super::rust;
use crate::{canister_method, errors::KybraResult, py_ast::PyAst};

impl PyAst {
    pub fn build_pre_upgrade_method(&self) -> KybraResult<Option<PreUpgradeMethod>> {
        let pre_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PreUpgrade);

        if pre_upgrade_function_defs.len() > 1 {
            return Err(pre_upgrade_function_defs
                .iter()
                .map(|pre_upgrade_function_def| {
                    pre_upgrade_function_def.only_one_pre_upgrade_method_allowed_error()
                })
                .collect());
        }

        let pre_upgrade_function_def_option = pre_upgrade_function_defs.get(0);

        if let Some(pre_upgrade_function_def) = pre_upgrade_function_def_option {
            if !canister_method::is_void(pre_upgrade_function_def.build_return_type()?) {
                return Err(pre_upgrade_function_def.post_upgrade_method_must_return_void_error());
            }
        }

        Ok(Some(PreUpgradeMethod {
            body: rust::generate(pre_upgrade_function_def_option)?,
            guard_function_name: None,
        }))
    }
}
