use super::rust;
use cdk_framework::{
    act::node::{
        canister_method::{CanisterMethodType, InitMethod},
        Param,
    },
    traits::CollectResults,
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    canister_method::{
        self,
        errors::{MultipleSystemMethods, ReturnTypeMustBeVoid},
    },
    method_utils::params::InternalOrExternal,
    py_ast::PyAst,
    Error,
};

impl PyAst {
    pub fn build_init_method(&self) -> Result<(InitMethod, Vec<Param>, TokenStream), Vec<Error>> {
        let init_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Init);

        if init_function_defs.len() > 1 {
            return Err(MultipleSystemMethods::err_from_stmt(
                &init_function_defs,
                CanisterMethodType::Init,
            )
            .into());
        }

        let init_function_def_option = init_function_defs.get(0);

        let (params, guard_function_name) =
            if let Some(init_function_def) = init_function_def_option {
                let (guard_function_name, params, return_type) = (
                    init_function_def
                        .get_guard_function_name()
                        .map_err(Error::into),
                    init_function_def.build_params(InternalOrExternal::Internal),
                    init_function_def.build_return_type(),
                )
                    .collect_results()?;

                if !canister_method::is_void(return_type) {
                    return Err(ReturnTypeMustBeVoid::err_from_stmt(
                        init_function_def,
                        CanisterMethodType::Init,
                    )
                    .into());
                }
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
