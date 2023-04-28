use cdk_framework::act::node::canister_method::{CanisterMethodType, PostUpgradeMethod};
use proc_macro2::TokenStream;
use quote::quote;

use super::rust;
use crate::{
    canister_method, errors::KybraResult, method_utils::params::InternalOrExternal, py_ast::PyAst,
};

impl PyAst {
    pub fn build_post_upgrade_method(&self) -> KybraResult<(PostUpgradeMethod, TokenStream)> {
        let post_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            return Err(post_upgrade_function_defs
                .iter()
                .map(|post_upgrade_function_def| {
                    post_upgrade_function_def.only_one_post_upgrade_method_allowed_error()
                })
                .collect());
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let (params, guard_function_name) = if let Some(post_upgrade_function_def) =
            post_upgrade_function_def_option
        {
            if !canister_method::is_void(post_upgrade_function_def.build_return_type()?) {
                return Err(post_upgrade_function_def.post_upgrade_method_must_return_void_error());
            }
            (
                post_upgrade_function_def.build_params(InternalOrExternal::Internal)?,
                post_upgrade_function_def.get_guard_function_name()?,
            )
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
                params: params.clone(),
                body: rust::generate(&params)?,
                guard_function_name,
            },
            call_to_post_upgrade_py_function,
        ))
    }
}
