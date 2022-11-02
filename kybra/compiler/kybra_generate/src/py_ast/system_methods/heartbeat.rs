use quote::quote;

use crate::py_ast::PyAst;
use cdk_framework::{nodes::ActHeartbeatMethod, CanisterMethodType};

impl PyAst<'_> {
    pub fn build_heartbeat_method(&self) -> Option<ActHeartbeatMethod> {
        let heartbeat_function_defs = self.get_function_def_of_type(CanisterMethodType::Heartbeat);

        if heartbeat_function_defs.len() > 1 {
            todo!();
        }

        let heartbeat_function_def_option = heartbeat_function_defs.get(0);

        if let Some(heartbeat_function_def) = heartbeat_function_def_option {
            let call_to_heartbeat_py_function =
                heartbeat_function_def.generate_call_to_py_function();
            let body = quote! {
                unsafe {
                    ic_cdk::spawn(async {
                        let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                        let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                        _kybra_interpreter.enter(|vm| {
                            #call_to_heartbeat_py_function
                        });
                        // TODO todo!("What about this azle azync result handler that is in the other heartbeat?")
                    });
                }
            };
            Some(ActHeartbeatMethod { body })
        } else {
            None
        }
    }
}
