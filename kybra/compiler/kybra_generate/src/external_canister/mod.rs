pub mod errors;

use cdk_framework::act::node::{external_canister::Method, ExternalCanister, ReturnType};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

use super::method_utils::params::InternalOrExternal;

impl PyAst {
    pub fn build_external_canisters(&self) -> KybraResult<Vec<ExternalCanister>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_external_canister())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn to_external_canister_method(&self, canister_name: &String) -> KybraResult<Method> {
        match &self.node {
            StmtKind::FunctionDef {
                name,
                decorator_list,
                ..
            } => {
                ensure_decorated_as_method_or_err(self, decorator_list, &canister_name, name)?;

                Ok(Method {
                    name: name.clone(),
                    params: self.build_params(InternalOrExternal::External)?,
                    return_type: ReturnType::new(self.build_return_type()?),
                })
            }
            _ => Err(self.class_with_not_function_defs_error(canister_name)),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn as_external_canister(&self) -> KybraResult<Option<ExternalCanister>> {
        if !self.is_external_canister() {
            return Ok(None);
        }
        match &self.node {
            StmtKind::ClassDef { name, body, .. } => {
                let method_results: Vec<_> = body
                    .iter()
                    .map(|located_statement| {
                        SourceMapped::new(located_statement, self.source_map.clone())
                            .to_external_canister_method(&name)
                    })
                    .collect();

                let methods = crate::errors::collect_kybra_results(method_results)?;

                if methods.len() == 0 {
                    return Err(self.class_must_have_methods_error(name));
                }

                Ok(Some(ExternalCanister {
                    name: name.clone(),
                    methods,
                }))
            }
            // We filter out any non classDefs in KybraProgram.get_external_canister_declarations
            _ => Err(crate::errors::unreachable()),
        }
    }

    pub fn is_external_canister(&self) -> bool {
        match &self.node {
            StmtKind::ClassDef { bases, .. } => bases.iter().fold(false, |acc, base| {
                let is_external_canister = match &base.node {
                    ExprKind::Name { id, .. } => id == "Canister",
                    _ => false,
                };
                acc || is_external_canister
            }),
            _ => false,
        }
    }
}

fn ensure_decorated_as_method_or_err(
    method_stmt: &SourceMapped<&Located<StmtKind>>,
    decorator_list: &Vec<Located<ExprKind>>,
    canister_name: &String,
    method_name: &String,
) -> KybraResult<()> {
    if decorator_list.len() == 0 {
        return Err(method_stmt.missing_decorator_error(canister_name, method_name));
    }

    if decorator_list.len() > 1 {
        return Err(method_stmt.too_many_decorators_error(canister_name, method_name));
    }

    match &decorator_list[0].node {
        ExprKind::Name { id, ctx: _ } => {
            if id != "update" || id != "query" {
                return Err(method_stmt.wrong_decorator_error(
                    canister_name,
                    method_name,
                    &id.to_string(),
                ));
            }
        }
        _ => {
            return Err(method_stmt.invalid_decorator_error(canister_name, method_name));
        }
    }

    Ok(())
}
