// TODO init_method/build.rs and post_upgrade_method/build.rs are almost identical

use super::rust;
use cdk_framework::{
    act::node::canister_method::{CanisterMethodType, InitMethod},
    traits::CollectResults,
};
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    canister_method::{
        self,
        errors::{MultipleSystemMethods, ReturnTypeMustBeVoid},
    },
    errors::CollectResults as OtherCollectResults,
    method_utils::params::InternalOrExternal,
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

impl PyAst {
    pub fn build_init_method(&self) -> Result<InitMethod, Vec<Error>> {
        let init_function_defs = self.get_canister_stmt_of_type(CanisterMethodType::Init);

        let init_methods = build_init_methods(&init_function_defs, &self.entry_module_name);

        if init_function_defs.len() > 1 {
            return Err(vec![
                MultipleSystemMethods::err_from_stmt(&init_function_defs, CanisterMethodType::Init)
                    .into(),
                init_methods.err().unwrap_or_default(),
            ]
            .concat());
        }

        Ok(match init_methods?.pop() {
            Some(init_method) => init_method,
            None => build_init_method(None, &self.entry_module_name)?,
        })
    }
}

fn build_init_methods(
    init_function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<Vec<InitMethod>, Vec<Error>> {
    init_function_defs
        .iter()
        .map(|init_function_def| build_init_method(Some(init_function_def), entry_module_name))
        .collect_results()
}

fn build_init_method(
    init_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<InitMethod, Vec<Error>> {
    match init_function_def_option {
        Some(init_function_def) => {
            let (params, return_type) = (
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

            Ok(InitMethod {
                params: params.clone(),
                body: rust::generate(init_function_def_option, entry_module_name)?,
            })
        }
        None => Ok(InitMethod {
            params: vec![],
            body: rust::generate(init_function_def_option, entry_module_name)?,
        }),
    }
}
