use super::rust;
use cdk_framework::act::node::{
    canister_method::{CanisterMethodType, InitMethod},
    Param,
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{canister_method, method_utils::params::InternalOrExternal, py_ast::PyAst, Error};

impl PyAst {
    pub fn build_init_method(&self) -> Result<(InitMethod, Vec<Param>, TokenStream), Vec<Error>> {
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
                    return Err(vec![init_function_def.init_method_must_return_void_error()]);
                }
                let params = init_function_def.build_params(InternalOrExternal::Internal)?;
                let guard_function_name = match init_function_def.get_guard_function_name() {
                    Ok(guard_function_name) => guard_function_name,
                    Err(err) => return Err(vec![err]),
                };
                (params, guard_function_name)
            } else {
                (vec![], None)
            };

        let call_to_init_py_function = match init_function_def_option {
            Some(init_function_def) => init_function_def.generate_call_to_py_function()?,
            None => quote!(),
        };

        Ok((
            InitMethod {
                params: params.clone(),
                body: rust::generate(&params),
                guard_function_name,
            },
            params,
            call_to_init_py_function,
        ))
    }
}
