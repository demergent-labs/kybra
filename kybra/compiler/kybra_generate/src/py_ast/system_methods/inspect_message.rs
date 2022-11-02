use quote::quote;

use crate::py_ast::PyAst;
use cdk_framework::{nodes::ActInspectMessageMethod, CanisterMethodType};

impl PyAst<'_> {
    pub fn build_inspect_method(&self) -> Option<ActInspectMessageMethod> {
        let inspect_message_function_defs =
            self.get_function_def_of_type(CanisterMethodType::InspectMessage);

        if inspect_message_function_defs.len() > 1 {
            todo!();
        }

        let inspect_message_function_def_option = inspect_message_function_defs.get(0);

        if let Some(inspect_method_function_def) = inspect_message_function_def_option {
            let name = inspect_method_function_def.get_function_name();

            let call_to_inspect_message_py_function =
                inspect_method_function_def.generate_call_to_py_function();

            let body = quote! {
                unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                    _kybra_interpreter.enter(|vm| {
                        #call_to_inspect_message_py_function
                    });
                }
            };
            Some(ActInspectMessageMethod { name, body })
        } else {
            None
        }
    }
}
