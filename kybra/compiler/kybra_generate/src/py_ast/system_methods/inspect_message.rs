use cdk_framework::act::node::canister_method::{CanisterMethodType, InspectMessageMethod};

use crate::{generators::canister_methods::inspect_message, py_ast::PyAst};

impl PyAst<'_> {
    pub fn build_inspect_method(&self) -> Option<InspectMessageMethod> {
        let inspect_message_function_defs =
            self.get_function_def_of_type(CanisterMethodType::InspectMessage);

        if inspect_message_function_defs.len() > 1 {
            todo!();
        }

        let inspect_message_function_def_option = inspect_message_function_defs.get(0);

        if let Some(inspect_method_function_def) = inspect_message_function_def_option {
            let body =
                inspect_message::generate_inspect_message_method_body(inspect_method_function_def);
            Some(InspectMessageMethod { body })
        } else {
            None
        }
    }
}
