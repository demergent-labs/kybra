use cdk_framework::{
    act::node::{
        canister_method::{CanisterMethod, CanisterMethodType, InitMethod, PostUpgradeMethod},
        Param,
    },
    traits::CollectResults,
};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    canister_method::{
        self,
        errors::{MultipleSystemMethods, ReturnTypeMustBeVoid},
        init_method::rust as InitMethodRust,
        post_upgrade_method::rust as PostUpgradeMethodRust,
    },
    errors::{CollectResults as OtherCollectResults, Unreachable},
    method_utils::params::InternalOrExternal,
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

impl PyAst {
    pub fn build_init_method(&self) -> Result<InitMethod, Vec<Error>> {
        let init_or_post_upgrade_method =
            self.build_init_or_post_upgrade_method(CanisterMethodType::Init)?;

        match init_or_post_upgrade_method {
            CanisterMethod::Init(init_method) => Ok(init_method),
            _ => Err(vec![Unreachable::error()]),
        }
    }

    pub fn build_init_or_post_upgrade_method(
        &self,
        canister_method_type: CanisterMethodType,
    ) -> Result<CanisterMethod, Vec<Error>> {
        let function_defs = self.get_canister_stmt_of_type(canister_method_type);

        let init_or_post_upgrade_methods = build_init_or_post_upgrade_methods(
            canister_method_type,
            &function_defs,
            &self.entry_module_name,
        );

        if function_defs.len() > 1 {
            return Err(vec![
                MultipleSystemMethods::err_from_stmt(&function_defs, canister_method_type).into(),
                init_or_post_upgrade_methods.err().unwrap_or_default(),
            ]
            .concat());
        }

        Ok(match init_or_post_upgrade_methods?.pop() {
            Some(init_or_post_upgrade_method) => init_or_post_upgrade_method,
            None => build_init_or_post_upgrade_method(
                canister_method_type,
                None,
                &self.entry_module_name,
            )?,
        })
    }
}

fn build_init_or_post_upgrade_methods(
    canister_method_type: CanisterMethodType,
    function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<Vec<CanisterMethod>, Vec<Error>> {
    function_defs
        .iter()
        .map(|function_def| {
            build_init_or_post_upgrade_method(
                canister_method_type,
                Some(function_def),
                entry_module_name,
            )
        })
        .collect_results()
}

fn build_init_or_post_upgrade_method(
    canister_method_type: CanisterMethodType,
    function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<CanisterMethod, Vec<Error>> {
    let params = get_params_and_check_return_type(function_def_option)?;

    match canister_method_type {
        CanisterMethodType::Init => Ok(CanisterMethod::Init(InitMethod {
            params,
            body: InitMethodRust::generate(function_def_option, entry_module_name)?,
        })),
        CanisterMethodType::PostUpgrade => Ok(CanisterMethod::PostUpgrade(PostUpgradeMethod {
            params,
            body: PostUpgradeMethodRust::generate(function_def_option, entry_module_name)?,
        })),
        _ => Err(vec![Unreachable::error()]),
    }
}

fn get_params_and_check_return_type(
    function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
) -> Result<Vec<Param>, Vec<Error>> {
    if let Some(function_def) = function_def_option {
        let (params, return_type) = (
            function_def.build_params(InternalOrExternal::Internal),
            function_def.build_return_type(),
        )
            .collect_results()?;

        if !canister_method::is_void(return_type) {
            return Err(ReturnTypeMustBeVoid::err_from_stmt(
                function_def,
                CanisterMethodType::Init,
            )
            .into());
        }

        Ok(params)
    } else {
        Ok(vec![])
    }
}
