use super::rust;
use cdk_framework::act::node::canister_method::{CanisterMethodType, InitMethod};

use crate::{
    canister_method, errors::KybraResult, method_utils::params::InternalOrExternal, py_ast::PyAst,
};

impl PyAst {
    pub fn build_init_method(&self) -> KybraResult<InitMethod> {
        let init_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Init);

        if init_function_defs.len() > 1 {
            return Err(init_function_defs
                .iter()
                .map(|init_function_def| init_function_def.only_one_init_allowed_error())
                .collect());
        }

        let init_function_def_option = init_function_defs.get(0);

        let (params, guard_function_name) =
            if let Some(init_function_def) = init_function_def_option {
                if !canister_method::is_void(init_function_def.build_return_type()?) {
                    return Err(init_function_def.init_method_must_return_void_error());
                }
                (
                    init_function_def.build_params(InternalOrExternal::Internal)?,
                    init_function_def.get_guard_function_name(),
                )
            } else {
                (vec![], None)
            };

        Ok(InitMethod {
            params,
            body: rust::generate(init_function_def_option, &self.entry_module_name)?,
            guard_function_name,
        })
    }
}
