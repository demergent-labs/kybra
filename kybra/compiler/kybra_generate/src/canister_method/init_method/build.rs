use super::rust;
use cdk_framework::act::node::canister_method::{CanisterMethodType, InitMethod};

use crate::{errors::KybraResult, method_utils::params::InternalOrExternal, py_ast::PyAst};

impl PyAst {
    pub fn build_init_method(&self) -> KybraResult<Option<InitMethod>> {
        let init_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Init);

        if init_function_defs.len() > 1 {
            return Err(init_function_defs
                .iter()
                .map(|init_function_def| init_function_def.only_one_init_allowed_error())
                .collect());
        }

        let init_function_def_option = init_function_defs.get(0);

        let params = match init_function_def_option {
            Some(init_function_def) => {
                init_function_def.build_params(InternalOrExternal::Internal)?
            }
            None => vec![],
        };

        let body = rust::generate(init_function_def_option, &self.entry_module_name)?;

        Ok(Some(InitMethod { params, body }))
    }
}
