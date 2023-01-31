use crate::{generators::canister_methods::heartbeat, py_ast::PyAst};
use cdk_framework::{act::node::canister_methods::HeartbeatMethod, CanisterMethodType};

impl PyAst<'_> {
    pub fn build_heartbeat_method(&self) -> Option<HeartbeatMethod> {
        let heartbeat_function_defs = self.get_function_def_of_type(CanisterMethodType::Heartbeat);

        if heartbeat_function_defs.len() > 1 {
            todo!();
        }

        let heartbeat_function_def_option = heartbeat_function_defs.get(0);

        if let Some(heartbeat_function_def) = heartbeat_function_def_option {
            let body = heartbeat::generate_heartbeat_method_body(heartbeat_function_def);
            Some(HeartbeatMethod { body })
        } else {
            None
        }
    }
}
