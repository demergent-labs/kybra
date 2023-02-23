use cdk_framework::act::node::canister_method::{CanisterMethodType, InitMethod};

use crate::{generators::canister_methods::init, kybra_ast::NewPyAst};

impl NewPyAst {
    pub fn build_init_method(&self) -> InitMethod {
        let init_function_defs = self.get_function_def_of_type(CanisterMethodType::Init);

        if init_function_defs.len() > 1 {
            todo!();
        }

        let init_function_def_option = init_function_defs.get(0);

        let params = match init_function_def_option {
            Some(init_function_def) => init_function_def.build_params(),
            None => vec![],
        };

        let body =
            init::generate_init_method_body(init_function_def_option, &self.entry_module_name);

        InitMethod { params, body }
    }
}
