pub mod errors;

use cdk_framework::act::node::{ExternalCanister, ExternalCanisterMethod};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};

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
    pub fn to_external_canister_method(
        &self,
        canister_name: &String,
    ) -> KybraResult<ExternalCanisterMethod> {
        match &self.node {
            StmtKind::FunctionDef { name, args, body: _, decorator_list, returns, type_comment: _ } => {
                ensure_decorated_as_method_or_panic(decorator_list, &canister_name, name);

                let params = SourceMapped::new(
                    args.as_ref(),
                    self.source_map.clone()
                ).to_param_list()
                    .unwrap_or_else(|e| panic!("{}.{} violates Kybra requirements: {:?}", canister_name, name, e) );

                let expr_kind = returns.as_ref().expect(&format!("{}.{} is missing a return type", canister_name, &name));

                let return_type = SourceMapped::new(expr_kind.as_ref(), self.source_map.clone()).to_data_type()?;

                Ok(ExternalCanisterMethod {
                    name: name.clone(),
                    params,
                    return_type,
                })
            },
            _ => panic!("class \"{}\" should only contain function definitions. Please remove everything else.", canister_name)
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
                    .map(|located_statement| SourceMapped::new(located_statement, self.source_map.clone()).to_external_canister_method(&name) )
                    .collect();

                let methods = crate::errors::collect_kybra_results(method_results)?;

                if methods.len() == 0 {
                    panic!("class \"{}\" doesn't have any methods. External canisters are required to expose at least one method.", name)
                }

                Ok(Some(ExternalCanister {
                    name: name.clone(),
                    methods,
                }))
            }
            // We filter out any non classDefs in KybraProgram.get_external_canister_declarations
            _ => panic!("Oops! Looks like we introduced a bug while refactoring. Please open a ticket at https://github.com/demergent-labs/kybra/issues/new"),
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

fn ensure_decorated_as_method_or_panic(
    decorator_list: &Vec<Located<ExprKind>>,
    canister_name: &String,
    method_name: &String,
) {
    let decorator_name = "@method";

    if decorator_list.len() == 0 {
        panic!(
            "{}.{} is missing a \"{}\" decorator. Please add it above the method",
            canister_name, method_name, decorator_name
        );
    }

    if decorator_list.len() > 1 {
        panic!(
            "{}.{} has too many decorators. Please remove all but \"{}\"",
            canister_name, method_name, decorator_name
        )
    }

    match &decorator_list[0].node {
        ExprKind::Name { id, ctx: _ } => {
            if id != "method" {
                panic!(
                    "{}.{} has the wrong decorator: expected \"{}\", got \"@{}\"",
                    canister_name, method_name, decorator_name, id
                )
            }
        }
        _ => panic!(
            "{}.{} has an invalid decorator. Change it to \"{}\"",
            canister_name, method_name, decorator_name
        ),
    }
}
