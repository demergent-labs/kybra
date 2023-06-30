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

        let init_methods = build_init_methods(&init_function_defs);

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
            None => build_init_method(None)?,
        })
    }
}

fn build_init_method(
    init_function_def: Option<&SourceMapped<&Located<StmtKind>>>,
) -> Result<InitMethod, Vec<Error>> {
    match init_function_def {
        Some(init_function_def) => {
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

            Ok(InitMethod {
                params: params.clone(),
                body: rust::generate(),
                guard_function_name,
            })
        }
        None => Ok(InitMethod {
            params: vec![],
            body: rust::generate(),
            guard_function_name: None,
        }),
    }
}

fn build_init_methods(
    init_function_defs: &Vec<SourceMapped<&Located<StmtKind>>>,
) -> Result<Vec<InitMethod>, Vec<Error>> {
    init_function_defs
        .iter()
        .map(|init_function_def| build_init_method(Some(init_function_def)))
        .collect_results()
}
