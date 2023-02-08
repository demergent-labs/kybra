use std::collections::{HashMap, HashSet};

use rustpython_parser::ast::{Constant, ExprKind, Mod, StmtKind};

use crate::source_map::SourceMap;
use cdk_framework::{
    act::node::{
        canister_method::{QueryMethod, UpdateMethod},
        DataType, FunctionGuard,
    },
    CanisterMethodType,
};

use super::KybraStmt;

mod build_external_canisters;
pub mod stable_b_tree_map_nodes;
pub use stable_b_tree_map_nodes::StableBTreeMapNode;

pub struct KybraProgram<'a> {
    pub program: &'a Mod,
    pub programs: &'a Vec<Mod>,
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
                        programs: self.programs,
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
                        programs: self.programs,
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
                        programs: self.programs,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_canister_method_stmt()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    programs: self.programs,
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
                        programs: self.programs,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_external_canister()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    programs: self.programs,
                    source_map: self.source_map,
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
                        programs: self.programs,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_stable_b_tree_map_node()
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    programs: self.programs,
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
                        programs: self.programs,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_canister_method_type(method_type.clone())
                })
                .map(|stmt_kind| KybraStmt {
                    stmt_kind,
                    programs: self.programs,
                    source_map: self.source_map,
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
                        programs: self.programs,
                        source_map: self.source_map,
                    }
                    .as_query_method()
                })
                .collect(),
            _ => vec![],
        }
    }

    fn get_guard_function_names(&self) -> Vec<String> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .map(|stmt_kind| match &stmt_kind.node {
                    StmtKind::FunctionDef { decorator_list, .. } => {
                        decorator_list
                            .iter()
                            .fold(None, |_, decorator| match &decorator.node {
                                ExprKind::Call { keywords, .. } => {
                                    keywords.iter().fold(None, |_, keyword| {
                                        if let Some(arg) = &keyword.node.arg {
                                            if arg == "guard" {
                                                match &keyword.node.value.node {
                                                    ExprKind::Constant { value, .. } => match value
                                                    {
                                                        Constant::Str(string) => {
                                                            Some(string.clone())
                                                        }
                                                        _ => None,
                                                    },
                                                    _ => None,
                                                }
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    })
                                }
                                _ => None,
                            })
                    }
                    _ => None,
                })
                .collect(),
            _ => vec![],
        }
        .iter()
        .fold(vec![], |acc, result| match result {
            Some(result) => {
                if !acc.contains(result) {
                    vec![acc, vec![result.clone()]].concat()
                } else {
                    acc
                }
            }
            None => acc,
        })
    }

    pub fn build_function_guard_act_nodes(&self) -> Vec<FunctionGuard> {
        let function_guard_names = self.get_guard_function_names();
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| match &stmt_kind.node {
                    StmtKind::FunctionDef { name, .. } => function_guard_names.contains(name),
                    _ => false,
                })
                .map(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        programs: self.programs,
                        source_map: self.source_map,
                    };
                    match kybra_stmt.as_function_guard() {
                        Some(function_guard) => function_guard,
                        None => panic!("Unreachable"),
                    }
                })
                .collect(),
            _ => vec![],
        }
    }
    pub fn build_update_method_act_nodes(&self) -> Vec<UpdateMethod> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter_map(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        programs: self.programs,
                        source_map: self.source_map,
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
                        programs: self.programs,
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
                        programs: self.programs,
                        source_map: self.source_map,
                    };
                    kybra_stmt.build_act_data_type()
                })
                .collect(),
            _ => vec![],
        }
    }
}

pub fn get_act_data_type_node_by_name(
    programs: &Vec<Mod>,
    source_map: &SourceMap,
    name: String,
) -> Result<DataType, String> {
    let matching_data_types: Vec<_> = programs
        .iter()
        .map(|program| match program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        programs,
                        source_map,
                    };
                    eprintln!(
                        "{} == {:?}? {}",
                        name.clone(),
                        kybra_stmt.get_alias_name(),
                        Some(name.clone()) == kybra_stmt.get_alias_name(),
                    );
                    match kybra_stmt.get_alias_name() {
                        Some(alias_name) => alias_name == name,
                        None => false,
                    }
                })
                .map(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        programs,
                        source_map,
                    };
                    eprintln!(
                        "This is the name we are looking at: {:?}",
                        kybra_stmt.get_alias_name()
                    );
                    kybra_stmt.build_act_data_type()
                })
                .collect(),
            _ => vec![],
        })
        .fold(vec![], |acc, list_of_data_types| {
            vec![acc, list_of_data_types].concat()
        });
    if matching_data_types.len() > 1 {
        return Err("We found more than one matching data type.".to_string());
    }
    if matching_data_types.len() == 0 {
        return Err("We didn't find a matching data type".to_string());
    }
    Ok(matching_data_types[0].clone())
}
