use std::collections::{HashMap, HashSet};

use rustpython_parser::ast::Mod;

use crate::source_map::SourceMap;
use cdk_framework::act::node::{
    canister_method::{CanisterMethodType, QueryMethod, UpdateMethod},
    DataType,
};

use super::KybraStmt;

mod build_external_canisters;
pub mod stable_b_tree_map_nodes;
pub use stable_b_tree_map_nodes::StableBTreeMapNode;

pub struct KybraProgram {
    pub program: Mod,
    pub source_map: SourceMap,
}

impl KybraProgram {
    pub fn generate_type_alias_lookup(&self) -> HashMap<String, KybraStmt> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
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
                        source_map: self.source_map.clone(),
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
                        source_map: self.source_map.clone(),
                    };
                    kybra_stmt.is_canister_method_stmt()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map.clone(),
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
                        source_map: self.source_map.clone(),
                    };
                    kybra_stmt.is_external_canister()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map.clone(),
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn get_kybra_stable_b_tree_node_stmts(&self) -> Vec<KybraStmt> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
                    };
                    kybra_stmt.is_stable_b_tree_map_node()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map.clone(),
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
                        source_map: self.source_map.clone(),
                    };
                    kybra_stmt.is_canister_method_type(method_type.clone())
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    source_map: self.source_map.clone(),
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn build_query_method_act_nodes(&self) -> Vec<QueryMethod> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter_map(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
                    }
                    .as_query_method()
                })
                .collect(),
            _ => vec![],
        }
    }

    // fn get_guard_function_names(&self) -> Vec<String> {
    //     match &self.program {
    //         Mod::Module { body, .. } => body
    //             .iter()
    //             .map(|stmt_kind| match &stmt_kind.node {
    //                 StmtKind::FunctionDef { decorator_list, .. } => {
    //                     decorator_list
    //                         .iter()
    //                         .fold(None, |_, decorator| match &decorator.node {
    //                             ExprKind::Call { keywords, .. } => {
    //                                 keywords.iter().fold(None, |_, keyword| {
    //                                     if let Some(arg) = &keyword.node.arg {
    //                                         if arg == "guard" {
    //                                             match &keyword.node.value.node {
    //                                                 ExprKind::Constant { value, .. } => match value
    //                                                 {
    //                                                     Constant::Str(string) => {
    //                                                         Some(string.clone())
    //                                                     }
    //                                                     _ => None,
    //                                                 },
    //                                                 _ => None,
    //                                             }
    //                                         } else {
    //                                             None
    //                                         }
    //                                     } else {
    //                                         None
    //                                     }
    //                                 })
    //                             }
    //                             _ => None,
    //                         })
    //                 }
    //                 _ => None,
    //             })
    //             .collect(),
    //         _ => vec![],
    //     }
    //     .iter()
    //     .fold(vec![], |acc, result| match result {
    //         Some(result) => {
    //             if !acc.contains(result) {
    //                 vec![acc, vec![result.clone()]].concat()
    //             } else {
    //                 acc
    //             }
    //         }
    //         None => acc,
    //     })
    // }

    // pub fn build_function_guard_act_nodes(&self) -> Vec<GuardFunction> {
    //     let function_guard_names = self.get_guard_function_names();
    //     match &self.program {
    //         Mod::Module { body, .. } => body
    //             .iter()
    //             .filter(|stmt_kind| match &stmt_kind.node {
    //                 StmtKind::FunctionDef { name, .. } => function_guard_names.contains(name),
    //                 _ => false,
    //             })
    //             .map(|stmt_kind| {
    //                 let kybra_stmt = KybraStmt {
    //                     stmt_kind,
    //                     source_map: self.source_map.clone(),
    //                 };
    //                 match kybra_stmt.as_function_guard() {
    //                     Some(function_guard) => function_guard,
    //                     None => panic!("Unreachable"),
    //                 }
    //             })
    //             .collect(),
    //         _ => vec![],
    //     }
    // }

    pub fn build_update_method_act_nodes(&self) -> Vec<UpdateMethod> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter_map(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
                    }
                    .as_update_method()
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn get_act_data_type_nodes(&self, dependencies: &HashSet<String>) -> Vec<DataType> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map.clone(),
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
                        source_map: self.source_map.clone(),
                    };
                    kybra_stmt.build_act_data_type()
                })
                .collect(),
            _ => vec![],
        }
    }
}
