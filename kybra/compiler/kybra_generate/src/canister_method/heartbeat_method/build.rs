use cdk_framework::act::node::canister_method::{CanisterMethodType, HeartbeatMethod};
use cdk_framework::traits::CollectResults;

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
    pub fn build_heartbeat_method(&self) -> Result<Option<HeartbeatMethod>, Vec<Error>> {
        let heartbeat_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Heartbeat);

        if heartbeat_function_defs.len() > 1 {
            return Err(MultipleSystemMethods::err_from_stmt(
                &heartbeat_function_defs,
                CanisterMethodType::Heartbeat,
            )
            .into());
        }

        let heartbeat_function_def_option = heartbeat_function_defs.get(0);

        Ok(
            if let Some(heartbeat_function_def) = heartbeat_function_def_option {
                let (body, return_type) = (
                    rust::generate(heartbeat_function_def).map_err(Error::into),
                    heartbeat_function_def.build_return_type(),
                )
                    .collect_results()?;

                if !canister_method::is_void(return_type) {
                    return Err(ReturnTypeMustBeVoid::err_from_stmt(
                        heartbeat_function_def,
                        CanisterMethodType::Heartbeat,
                    )
                    .into());
                }

                Some(HeartbeatMethod { body })
            } else {
                None
            },
        )
    }
}
