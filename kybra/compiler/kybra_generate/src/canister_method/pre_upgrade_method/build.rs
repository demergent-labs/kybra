use cdk_framework::act::node::canister_method::{CanisterMethodType, PreUpgradeMethod};

use super::rust;
use crate::{canister_method, py_ast::PyAst, Error};

impl PyAst {
    pub fn build_pre_upgrade_method(&self) -> Result<Option<PreUpgradeMethod>, Vec<Error>> {
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

        Ok(
            if let Some(pre_upgrade_function_def) = pre_upgrade_function_def_option {
                if !canister_method::is_void(pre_upgrade_function_def.build_return_type()?) {
                    return Err(vec![
                        pre_upgrade_function_def.post_upgrade_method_must_return_void_error()
                    ]);
                }
                let guard_function_name = match pre_upgrade_function_def.get_guard_function_name() {
                    Ok(guard_function_name) => guard_function_name,
                    Err(err) => return Err(vec![err]),
                };
                Some(PreUpgradeMethod {
                    body: rust::generate(pre_upgrade_function_def)?,
                    guard_function_name,
                })
            } else {
                None
            },
        )
    }
}
