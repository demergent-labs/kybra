use cdk_framework::{
    act::node::{
        canister_method::{CanisterMethodType, PostUpgradeMethod},
        Param,
    },
    traits::CollectResults,
};
use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use super::rust;
use crate::{
    canister_method::{
        self,
        errors::{MultipleSystemMethods, ReturnTypeMustBeVoid},
    },
    method_utils::params::InternalOrExternal,
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

impl PyAst {
    pub fn build_post_upgrade_method(
        &self,
    ) -> Result<(PostUpgradeMethod, TokenStream), Vec<Error>> {
        let init_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Init);
        let post_upgrade_function_defs =
            self.get_canister_stmt_of_type(CanisterMethodType::PostUpgrade);
        let (
            init_params,
            (post_upgrade_params, guard_function_name),
            call_to_post_upgrade_py_function,
        ) = (
            extract_init_function(&init_function_defs),
            extract_post_upgrade_function(&post_upgrade_function_defs),
            generate_call(&post_upgrade_function_defs.get(0)),
        )
            .collect_results()?;
        let post_upgrade_method_params =
            select_post_upgrade_method_params(init_params, post_upgrade_params);
        let post_upgrade_method_body = generate_post_upgrade_method_body(
            &self.entry_module_name,
            init_function_defs.get(0),
            post_upgrade_function_defs.get(0),
        );

        Ok((
            PostUpgradeMethod {
                params: post_upgrade_method_params,
                body: post_upgrade_method_body,
                guard_function_name,
            },
            call_to_post_upgrade_py_function,
        ))
    }
}

fn extract_init_function(
    init_function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
) -> Result<Vec<Param>, Vec<Error>> {
    handle_multiple_function_defs(init_function_defs, CanisterMethodType::Init)?;

    match init_function_defs.get(0) {
        Some(init_function_def) => init_function_def.build_params(InternalOrExternal::Internal),
        None => Ok(vec![]),
    }
}

fn extract_post_upgrade_function(
    post_upgrade_function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
) -> Result<(Vec<Param>, Option<String>), Vec<Error>> {
    handle_multiple_function_defs(post_upgrade_function_defs, CanisterMethodType::PostUpgrade)?;

    match post_upgrade_function_defs.get(0) {
        Some(post_upgrade_function_def) => {
            extract_post_upgrade_params_and_guard_name(post_upgrade_function_def)
        }
        None => Ok((vec![], None)),
    }
}

fn extract_post_upgrade_params_and_guard_name(
    post_upgrade_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<(Vec<Param>, Option<String>), Vec<Error>> {
    let (post_upgrade_params, guard_function_name, _) = (
        post_upgrade_function_def.build_params(InternalOrExternal::Internal),
        post_upgrade_function_def
            .get_guard_function_name()
            .map_err(Error::into),
        validate_post_upgrade_return_type(post_upgrade_function_def),
    )
        .collect_results()?;

    Ok((post_upgrade_params, guard_function_name))
}

fn select_post_upgrade_method_params(
    init_params: Vec<Param>,
    post_upgrade_params: Vec<Param>,
) -> Vec<Param> {
    if post_upgrade_params.is_empty() {
        init_params
    } else {
        post_upgrade_params
    }
}

fn generate_post_upgrade_method_body(
    entry_module_name: &String,
    init_function_def: Option<&SourceMapped<&Located<StmtKind>>>,
    post_upgrade_function_def: Option<&SourceMapped<&Located<StmtKind>>>,
) -> TokenStream {
    rust::generate(
        init_function_def,
        post_upgrade_function_def,
        entry_module_name,
    )
}

fn handle_multiple_function_defs(
    function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
    canister_method_type: CanisterMethodType,
) -> Result<(), Vec<Error>> {
    if function_defs.len() > 1 {
        Err(MultipleSystemMethods::err_from_stmt(&function_defs, canister_method_type).into())
    } else {
        Ok(())
    }
}

fn validate_post_upgrade_return_type(
    post_upgrade_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<(), Vec<Error>> {
    let return_type = post_upgrade_function_def.build_return_type()?;

    if !canister_method::is_void(return_type) {
        Err(ReturnTypeMustBeVoid::err_from_stmt(
            post_upgrade_function_def,
            CanisterMethodType::PostUpgrade,
        )
        .into())
    } else {
        Ok(())
    }
}

fn generate_call(
    post_upgrade_function_def_option: &Option<&SourceMapped<&Located<StmtKind>>>,
) -> Result<TokenStream, Vec<Error>> {
    match &post_upgrade_function_def_option {
        Some(post_upgrade_function_def) => post_upgrade_function_def.generate_call_to_py_function(),
        None => Ok(quote!()),
    }
}
