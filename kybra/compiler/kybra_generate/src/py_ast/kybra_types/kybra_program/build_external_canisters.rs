use cdk_framework::nodes::ActExternalCanister;
use rustpython_parser::ast::Mod;

use super::KybraProgram;
use crate::py_ast::KybraStmt;

impl KybraProgram<'_> {
    pub fn build_external_canisters(&self) -> Vec<ActExternalCanister> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    }
                    .is_external_canister()
                })
                .map(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    }
                    .to_act_external_canister()
                })
                .collect(),
            _ => vec![],
        }
    }
}
