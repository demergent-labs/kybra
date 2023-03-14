use cdk_framework::act::node::canister_method::{CanisterMethodType, InspectMessageMethod};

use super::rust;
use crate::{errors::KybraResult, py_ast::PyAst};

impl PyAst {
    pub fn build_inspect_method(&self) -> KybraResult<Option<InspectMessageMethod>> {
        let inspect_message_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::InspectMessage);

        if inspect_message_function_defs.len() > 1 {
            return Err(inspect_message_function_defs
                .iter()
                .map(|inspect_function_def| {
                    inspect_function_def.only_one_inspect_message_method_allowed_error()
                })
                .collect());
        }

        let inspect_message_function_def_option = inspect_message_function_defs.get(0);

        Ok(
            if let Some(inspect_method_function_def) = inspect_message_function_def_option {
                Some(InspectMessageMethod {
                    body: rust::generate(inspect_method_function_def)?,
                    guard_function_name: None,
                })
            } else {
                None
            },
        )
    }
}
