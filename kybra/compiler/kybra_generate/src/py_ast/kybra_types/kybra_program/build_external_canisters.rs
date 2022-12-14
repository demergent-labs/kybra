use cdk_framework::act::node::ExternalCanister;
use rustpython_parser::ast::Mod;

use super::KybraProgram;
use crate::py_ast::KybraStmt;

impl KybraProgram {
    pub fn build_external_canisters(&self) -> Vec<ExternalCanister> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
                    }
                    .is_external_canister()
                })
                .map(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
                    }
                    .to_act_external_canister()
                })
                .collect(),
            _ => vec![],
        }
    }
}
