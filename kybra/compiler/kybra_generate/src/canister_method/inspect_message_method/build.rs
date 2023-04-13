use cdk_framework::act::node::canister_method::{CanisterMethodType, InspectMessageMethod};

use super::rust;
use crate::{canister_method, py_ast::PyAst, Error};

impl PyAst {
    pub fn build_inspect_method(&self) -> Result<Option<InspectMessageMethod>, Vec<Error>> {
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
                match inspect_method_function_def.build_return_type() {
                    Ok(return_type) => {
                        if !canister_method::is_void(return_type) {
                            return Err(vec![inspect_method_function_def
                                .inspect_message_method_must_return_void_error()]);
                        }
                    }
                    Err(_) => todo!(),
                }
                let guard_function_name =
                    match inspect_method_function_def.get_guard_function_name() {
                        Ok(guard_function_name) => guard_function_name,
                        Err(err) => return Err(vec![err]),
                    };
                Some(InspectMessageMethod {
                    body: rust::generate(inspect_method_function_def)?,
                    guard_function_name,
                })
            } else {
                None
            },
        )
    }
}
