use super::rust;
use cdk_framework::act::node::{
    canister_method::{CanisterMethodType, InitMethod},
    Param,
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    canister_method, errors::KybraResult, method_utils::params::InternalOrExternal, py_ast::PyAst,
};

impl PyAst {
    pub fn build_init_method(&self) -> KybraResult<(InitMethod, Vec<Param>, TokenStream)> {
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
                    init_function_def.get_guard_function_name()?,
                )
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
                body: rust::generate(&params)?,
                guard_function_name,
            },
            params,
            call_to_init_py_function,
        ))
    }
}
