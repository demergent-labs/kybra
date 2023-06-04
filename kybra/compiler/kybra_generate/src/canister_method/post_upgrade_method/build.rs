use cdk_framework::{
    act::node::canister_method::{CanisterMethodType, PostUpgradeMethod},
    traits::CollectResults,
};
use proc_macro2::TokenStream;
use quote::quote;

use super::rust;
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
    pub fn build_post_upgrade_method(
        &self,
    ) -> Result<(PostUpgradeMethod, TokenStream), Vec<Error>> {
        let init_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Init);

        if init_function_defs.len() > 1 {
            return Err(MultipleSystemMethods::err_from_stmt(
                &init_function_defs,
                CanisterMethodType::Init,
            )
            .into());
        }

        let init_function_def_option = init_function_defs.get(0);

        let init_params = match init_function_def_option {
            Some(init_function_def) => init_function_def.build_params(InternalOrExternal::Internal),
            None => Ok(vec![]),
        }?;

        let post_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            return Err(MultipleSystemMethods::err_from_stmt(
                &post_upgrade_function_defs,
                CanisterMethodType::Heartbeat,
            )
            .into());
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let (post_upgrade_params, guard_function_name) =
            if let Some(post_upgrade_function_def) = post_upgrade_function_def_option {
                let (guard_function_name, post_upgrade_params, return_type) = (
                    post_upgrade_function_def
                        .get_guard_function_name()
                        .map_err(Error::into),
                    post_upgrade_function_def.build_params(InternalOrExternal::Internal),
                    post_upgrade_function_def.build_return_type(),
                )
                    .collect_results()?;

                if !canister_method::is_void(return_type) {
                    return Err(ReturnTypeMustBeVoid::err_from_stmt(
                        post_upgrade_function_def,
                        CanisterMethodType::PostUpgrade,
                    )
                    .into());
                }

                (post_upgrade_params, guard_function_name)
            } else {
                (vec![], None)
            };

        let call_to_post_upgrade_py_function = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => {
                post_upgrade_function_def.generate_call_to_py_function()?
            }
            None => quote!(),
        };

        Ok((
            PostUpgradeMethod {
                params: if post_upgrade_params.len() == 0 {
                    init_params
                } else {
                    post_upgrade_params
                },
                body: rust::generate(
                    init_function_def_option,
                    post_upgrade_function_def_option,
                    &self.entry_module_name,
                ),
                guard_function_name,
            },
            call_to_post_upgrade_py_function,
        ))
    }
}
