use cdk_framework::{
    nodes::{ActExternalCanister, ActExternalCanisterMethod},
    ToActDataType,
};
use rustpython_parser::ast::{ExprKind, StmtKind};

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
                            StmtKind::FunctionDef { name, args, body: _, decorator_list: _, returns, type_comment: _ } => {
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
