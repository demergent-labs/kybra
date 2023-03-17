use cdk_framework::act::node::canister_method::{CanisterMethodType, HeartbeatMethod};

use super::rust;
use crate::{canister_method, errors::KybraResult, py_ast::PyAst};

impl PyAst {
    pub fn build_heartbeat_method(&self) -> KybraResult<Option<HeartbeatMethod>> {
        let heartbeat_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Heartbeat);

        if heartbeat_function_defs.len() > 1 {
            return Err(heartbeat_function_defs
                .iter()
                .map(|heartbeat_function_def| {
                    heartbeat_function_def.only_one_heartbeat_allowed_error()
                })
                .collect());
        }

        let heartbeat_function_def_option = heartbeat_function_defs.get(0);

        Ok(
            if let Some(heartbeat_function_def) = heartbeat_function_def_option {
                if !canister_method::is_void(heartbeat_function_def.build_return_type()?) {
                    return Err(heartbeat_function_def.heartbeat_method_must_return_void_error());
                }
                let body = rust::generate(heartbeat_function_def)?;
                Some(HeartbeatMethod {
                    body,
                    guard_function_name: None,
                })
            } else {
                None
            },
        )
    }
}
