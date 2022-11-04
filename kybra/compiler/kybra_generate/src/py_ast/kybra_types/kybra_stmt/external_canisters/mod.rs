use cdk_framework::{
    nodes::{ActExternalCanister, ActExternalCanisterMethod},
    ToActDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use super::KybraStmt;
use crate::py_ast::kybra_types::{KybraArguments, KybraExpr};

impl KybraStmt<'_> {
    pub fn to_act_external_canister(&self) -> ActExternalCanister {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { name, body, .. } => {
                let canister_name = name.clone();
                let methods: Vec<ActExternalCanisterMethod> = body
                    .iter()
                    .map(|located_statement| -> ActExternalCanisterMethod {
                        match &located_statement.node {
                            StmtKind::FunctionDef { name, args, body: _, decorator_list, returns, type_comment: _ } => {
                                ensure_decorated_as_method_or_panic(decorator_list, &canister_name, name);

                                let params = KybraArguments {
                                    arguments: args.as_ref(),
                                    source_map: self.source_map
                                }.to_act_fn_params()
                                 .unwrap_or_else(|e| panic!("{}.{} violates Kybra requirements: {}", canister_name, name, e) );

                                let expr_kind = returns.as_ref().expect(&format!("{}.{} is missing a return type", canister_name, &name));

                                let return_type = KybraExpr {
                                    located_expr: expr_kind,
                                    source_map: self.source_map,
                                }.to_act_data_type(&None);

                                ActExternalCanisterMethod {
                                    name: name.clone(),
                                    params,
                                    return_type,
                                }
                            },
                            _ => panic!("class \"{}\" should only contain function definitions. Please remove everything else.", name)
                        }
                    })
                    .collect();

                if methods.len() == 0 {
                    panic!("class \"{}\" doesn't have any methods. External canisters are required to expose at least one method.", name)
                }

                ActExternalCanister {
                    name: canister_name,
                    methods,
                }
            }
            // We filter out any non classDefs in KybraProgram.get_external_canister_declarations
            _ => panic!("Oops! Looks like we introduced a bug while refactoring. Please open a ticket at https://github.com/demergent-labs/azle/issues/new"),
        }
    }

    pub fn is_external_canister(&self) -> bool {
        match &self.stmt_kind.node {
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
