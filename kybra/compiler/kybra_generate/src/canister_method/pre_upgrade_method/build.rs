use cdk_framework::{
    act::node::canister_method::{CanisterMethodType, PreUpgradeMethod},
    traits::CollectResults,
};

use super::rust;
use crate::{
    canister_method::{
        self,
        errors::{MultipleSystemMethods, ReturnTypeMustBeVoid},
    },
    py_ast::PyAst,
    Error,
};

impl PyAst {
    pub fn build_pre_upgrade_method(&self) -> Result<Option<PreUpgradeMethod>, Vec<Error>> {
        let pre_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PreUpgrade);

        if pre_upgrade_function_defs.len() > 1 {
            return Err(MultipleSystemMethods::err_from_stmt(
                &pre_upgrade_function_defs,
                CanisterMethodType::PreUpgrade,
            )
            .into());
        }

        let pre_upgrade_function_def_option = pre_upgrade_function_defs.get(0);

        Ok(
            if let Some(pre_upgrade_function_def) = pre_upgrade_function_def_option {
                let (body, return_type) = (
                    rust::generate(pre_upgrade_function_def),
                    pre_upgrade_function_def.build_return_type(),
                )
                    .collect_results()?;

                if !canister_method::is_void(return_type) {
                    return Err(ReturnTypeMustBeVoid::err_from_stmt(
                        pre_upgrade_function_def,
                        CanisterMethodType::Heartbeat,
                    )
                    .into());
                }

                Some(PreUpgradeMethod { body })
            } else {
                None
            },
        )
    }
}
