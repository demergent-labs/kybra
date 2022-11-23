use std::collections::{HashMap, HashSet};

use rustpython_parser::ast::Mod;

use crate::source_map::SourceMap;
use cdk_framework::{ActCanisterMethod, ActDataType, CanisterMethodType};

use super::KybraStmt;

mod build_external_canisters;

pub struct KybraProgram<'a> {
    pub program: Mod,
    pub source_map: &'a SourceMap,
}

impl KybraProgram<'_> {
    pub fn generate_type_alias_lookup(&self) -> HashMap<String, KybraStmt> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_record()
                        || kybra_stmt.is_tuple()
                        || kybra_stmt.is_variant()
                        || kybra_stmt.is_func()
                        || kybra_stmt.is_type_alias()
                })
                .fold(HashMap::new(), |mut acc, stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    let type_alias_name = kybra_stmt.get_name();
                    if let Some(type_alias_name) = type_alias_name {
                        acc.insert(type_alias_name, kybra_stmt);
                    }
                    acc
                }),
            _ => HashMap::new(),
        }
    }

    pub fn get_kybra_canister_method_stmts(&self) -> Vec<KybraStmt> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_canister_method_stmt()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map,
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn get_kybra_canister_stmts(&self) -> Vec<KybraStmt> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_external_canister()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map,
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn get_function_defs_of_type(&self, method_type: CanisterMethodType) -> Vec<KybraStmt> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_canister_method_type(method_type.clone())
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map,
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn build_canister_method_act_nodes(&self) -> Vec<ActCanisterMethod> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_canister_method_type(CanisterMethodType::Query)
                        || kybra_stmt.is_canister_method_type(CanisterMethodType::Update)
                })
                .map(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    match kybra_stmt.as_canister_method() {
                        Some(canister_method) => {
                            if kybra_stmt.is_canister_method_type(CanisterMethodType::Query) {
                                ActCanisterMethod::QueryMethod(canister_method)
                            } else {
                                ActCanisterMethod::UpdateMethod(canister_method)
                            }
                        }
                        None => panic!("Unreachable"),
                    }
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn get_act_data_type_nodes(&self, dependencies: &HashSet<String>) -> Vec<ActDataType> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    match kybra_stmt.get_alias_name() {
                        Some(alias_name) => {
                            if dependencies.contains(&alias_name) {
                                kybra_stmt.is_record()
                                    || kybra_stmt.is_tuple()
                                    || kybra_stmt.is_variant()
                                    || kybra_stmt.is_func()
                                    || kybra_stmt.is_type_alias()
                            } else {
                                false
                            }
                        }
                        None => false,
                    }
                })
                .map(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.build_act_data_type()
                })
                .collect(),
            _ => vec![],
        }
    }
}
