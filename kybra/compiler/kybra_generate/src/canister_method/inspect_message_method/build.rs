use cdk_framework::{
    act::node::canister_method::{CanisterMethodType, InspectMessageMethod},
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
    pub fn build_inspect_method(&self) -> Result<Option<InspectMessageMethod>, Vec<Error>> {
        let inspect_message_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::InspectMessage);

        if inspect_message_function_defs.len() > 1 {
            return Err(MultipleSystemMethods::err_from_stmt(
                &inspect_message_function_defs,
                CanisterMethodType::InspectMessage,
            )
            .into());
        }

        let inspect_message_function_def_option = inspect_message_function_defs.get(0);

        Ok(
            if let Some(inspect_method_function_def) = inspect_message_function_def_option {
                let (body, return_type) = (
                    rust::generate(inspect_method_function_def),
                    inspect_method_function_def.build_return_type(),
                )
                    .collect_results()?;

                if !canister_method::is_void(return_type) {
                    return Err(ReturnTypeMustBeVoid::err_from_stmt(
                        inspect_method_function_def,
                        CanisterMethodType::InspectMessage,
                    )
                    .into());
                }
                Some(InspectMessageMethod { body })
            } else {
                None
            },
        )
    }
}
