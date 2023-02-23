use cdk_framework::act::node::{
    canister_method::{InitMethod, PostUpgradeMethod, PreUpgradeMethod, QueryMethod, UpdateMethod},
    DataType, ExternalCanister,
};
use std::collections::{HashMap, HashSet};

use crate::{
    generators::body,
    py_ast::{
        kybra_types::{KybraStmt, StableBTreeMapNode},
        traits::GetDependencies,
    },
};

pub use self::kybra_ast::KybraAst;
use self::kybra_types::KybraProgram;

pub mod analyze;
mod kybra_ast;
pub mod kybra_types;
mod system_methods;
pub mod traits;
pub mod what_is_it;

pub struct PyAst {
    pub kybra_programs: Vec<KybraProgram>,
    pub entry_module_name: String,
}

impl PyAst {
    pub fn get_dependencies(&self) -> HashSet<String> {
        let kybra_canister_method_stmts = self.get_kybra_canister_method_stmts();
        let kybra_canister_stmts = self.get_kybra_canister_stmts();
        let stable_b_tree_nodes = self.get_kybra_stable_b_tree_node_stmts();
        let type_alias_lookup = self.generate_type_alias_lookup();

        let canister_method_dependencies = kybra_canister_method_stmts.iter().fold(
            HashSet::new(),
            |acc, kybra_canister_method_stmt| {
                acc.union(&kybra_canister_method_stmt.get_dependent_types(&type_alias_lookup, &acc))
                    .cloned()
                    .collect()
            },
        );
        let canister_dependencies =
            kybra_canister_stmts
                .iter()
                .fold(HashSet::new(), |acc, kybra_canister_stmt| {
                    acc.union(&kybra_canister_stmt.get_dependent_types(&type_alias_lookup, &acc))
                        .cloned()
                        .collect()
                });
        let stable_b_tree_map_dependencies =
            stable_b_tree_nodes
                .iter()
                .fold(HashSet::new(), |acc, stable_b_tree_map_node| {
                    acc.union(&stable_b_tree_map_node.get_dependent_types(&type_alias_lookup, &acc))
                        .cloned()
                        .collect()
                });

        canister_method_dependencies
            .union(
                &canister_dependencies
                    .union(&stable_b_tree_map_dependencies)
                    .cloned()
                    .collect(),
            )
            .cloned()
            .collect()
    }

    pub fn generate_type_alias_lookup(&self) -> HashMap<String, KybraStmt> {
        self.kybra_programs
            .iter()
            .fold(HashMap::new(), |mut acc, kybra_program| {
                acc.extend(kybra_program.generate_type_alias_lookup());
                acc
            })
    }

    pub fn to_kybra_ast(&self) -> KybraAst {
        let update_methods = self.build_update_methods();
        let query_methods = self.build_query_methods();
        let external_canisters = self.build_external_canisters();
        let canister_types = self.build_canister_types();
        let stable_b_tree_map_nodes = self.build_stable_b_tree_map_nodes();

        let rust_code = body::generate(
            &update_methods,
            &query_methods,
            &external_canisters,
            &stable_b_tree_map_nodes,
        );

        KybraAst {
            init_method: InitMethod {
                params: vec![],
                body: quote::quote!(),
            },
            pre_upgrade: PreUpgradeMethod {
                body: quote::quote!(),
            },
            post_upgrade: PostUpgradeMethod {
                params: vec![],
                body: quote::quote!(),
            },
            inspect_method: None,
            heartbeat: None,
            function_guards: vec![],
            external_canisters,
            rust_code,
            query_methods,
            update_methods,
            canister_types,
        }
    }

    fn get_kybra_canister_method_stmts(&self) -> Vec<KybraStmt> {
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.get_kybra_canister_method_stmts()].concat()
            })
    }

    fn get_kybra_canister_stmts(&self) -> Vec<KybraStmt> {
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.get_kybra_canister_stmts()].concat()
            })
    }

    fn get_kybra_stable_b_tree_node_stmts(&self) -> Vec<KybraStmt> {
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.get_kybra_stable_b_tree_node_stmts()].concat()
            })
    }

    fn build_update_methods(&self) -> Vec<UpdateMethod> {
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.build_update_method_act_nodes()].concat()
            })
    }

    // fn build_function_guards(&self) -> Vec<GuardFunction> {
    //     self.kybra_programs
    //         .iter()
    //         .fold(vec![], |acc, kybra_program| {
    //             vec![acc, kybra_program.build_function_guard_act_nodes()].concat()
    //         })
    // }

    fn build_query_methods(&self) -> Vec<QueryMethod> {
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.build_query_method_act_nodes()].concat()
            })
    }

    fn build_canister_types(&self) -> Vec<DataType> {
        let dependencies = self.get_dependencies();
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.get_act_data_type_nodes(&dependencies)].concat()
            })
    }

    // fn get_function_def_of_type(&self, method_type: CanisterMethodType) -> Vec<KybraStmt> {
    //     self.kybra_programs
    //         .iter()
    //         .fold(vec![], |mut acc, kybra_program| {
    //             acc.extend(kybra_program.get_function_defs_of_type(method_type.clone()));
    //             acc
    //         })
    // }

    fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        self.kybra_programs
            .iter()
            .map(|program| program.build_stable_b_tree_map_nodes())
            .collect::<Vec<_>>()
            .concat()
    }

    fn build_external_canisters(&self) -> Vec<ExternalCanister> {
        self.kybra_programs
            .iter()
            .map(|program| program.build_external_canisters())
            .collect::<Vec<_>>()
            .concat()
    }
}
