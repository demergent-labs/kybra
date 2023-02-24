use cdk_framework::{
    act::node::{ExternalCanister, ExternalCanisterMethod},
    ToDataType,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    py_ast::PyAst,
    source_map::SourceMapped,
};

impl PyAst {
    pub fn build_external_canisters(&self) -> Vec<ExternalCanister> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|stmt| stmt.to_act_external_canister().ok())
            .collect()
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn to_act_external_canister(&self) -> Result<ExternalCanister, Message> {
        if !self.is_external_canister() {
            return Err(self.create_error_message("Not a external canister", "", None));
        }
        match &self.node.node {
            StmtKind::ClassDef { name, body, .. } => {
                let canister_name = name.clone();
                let methods: Vec<ExternalCanisterMethod> = body
                    .iter()
                    .map(|located_statement| -> ExternalCanisterMethod {
                        match &located_statement.node {
                            StmtKind::FunctionDef { name, args, body: _, decorator_list, returns, type_comment: _ } => {
                                ensure_decorated_as_method_or_panic(decorator_list, &canister_name, name);

                                let params = SourceMapped {
                                    node: args.as_ref(),
                                    source_map: self.source_map.clone()
                                }.to_act_fn_params()
                                 .unwrap_or_else(|e| panic!("{}.{} violates Kybra requirements: {}", canister_name, name, e) );

                                let expr_kind = returns.as_ref().expect(&format!("{}.{} is missing a return type", canister_name, &name));

                                let return_type = SourceMapped {
                                    node: expr_kind.as_ref(),
                                    source_map: self.source_map.clone(),
                                }.to_data_type();

                                ExternalCanisterMethod {
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

                Ok(ExternalCanister {
                    name: canister_name,
                    methods,
                })
            }
            // We filter out any non classDefs in KybraProgram.get_external_canister_declarations
            _ => panic!("Oops! Looks like we introduced a bug while refactoring. Please open a ticket at https://github.com/demergent-labs/kybra/issues/new"),
        }
    }

    pub fn is_external_canister(&self) -> bool {
        match &self.node.node {
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
