use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

use crate::generators::{
    async_result_handler::generate_async_result_handler, kybra_serde::generate_kybra_serde,
    rng_seed, stable_b_tree_map::generate_stable_b_tree_map,
};
use cdk_framework::{
    nodes::{act_canister_method, data_type_nodes, ActExternalCanister},
    ActCanisterMethod, ActDataType, CanisterMethodType,
};

pub use self::{kybra_ast::KybraAst, kybra_types::KybraProgram};
use self::{
    kybra_types::{KybraStmt, StableBTreeMapNode},
    traits::GetDependencies,
};

mod kybra_ast;
pub mod kybra_types;
mod system_methods;
pub mod traits;
mod what_is_it;

pub struct PyAst<'a> {
    pub kybra_programs: Vec<KybraProgram<'a>>,
    pub entry_module_name: String,
    pub header: TokenStream,
}

impl PyAst<'_> {
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
        let canister_types = self.build_canister_types();
        let types_alias_inline_types = data_type_nodes::build_inline_type_acts(&canister_types);
        let canister_methods = self.build_canister_methods();
        let canister_method_types =
            act_canister_method::get_all_types_from_canister_method_acts(&canister_methods);
        let canister_method_inline_types =
            data_type_nodes::build_inline_type_acts(&canister_method_types);

        let all_types = vec![
            canister_types,
            types_alias_inline_types,
            canister_method_inline_types,
        ]
        .concat();

        let external_canisters = self.build_external_canisters();

        let stable_b_tree_map_nodes = self.build_stable_b_tree_map_nodes();

        let async_result_handler = generate_async_result_handler(&external_canisters);

        let kybra_serde = generate_kybra_serde();

        let stable_b_tree_map = generate_stable_b_tree_map(&stable_b_tree_map_nodes);

        let rng_seed = rng_seed::generate();

        let rust_code = quote! {
            pub fn _kybra_unwrap_rust_python_result<T>(
                rust_python_result: Result<T, PyRef<PyBaseException>>,
                vm: &rustpython::vm::VirtualMachine
            ) -> T {
                match rust_python_result {
                    Ok(ok) => ok,
                    Err(err) => {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                        panic!("{}", err_string);
                    },
                }
            }

            #async_result_handler

            #kybra_serde

            #stable_b_tree_map

            #rng_seed
        };

        KybraAst {
            init_method: self.build_init_method(
                &canister_methods,
                &external_canisters,
                &stable_b_tree_map_nodes,
            ),
            pre_upgrade: self.build_pre_upgrade_method(),
            post_upgrade: self.build_post_upgrade_method(
                &canister_methods,
                &external_canisters,
                &stable_b_tree_map_nodes,
            ),
            inspect_method: self.build_inspect_method(),
            heartbeat: self.build_heartbeat_method(),
            canister_types: all_types,
            canister_methods: self.build_canister_methods(),
            external_canisters,
            rust_code,
            header: self.header.clone(),
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

    fn build_canister_methods(&self) -> Vec<ActCanisterMethod> {
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.build_canister_method_act_nodes()].concat()
            })
    }

    fn build_canister_types(&self) -> Vec<ActDataType> {
        let dependencies = self.get_dependencies();
        self.kybra_programs
            .iter()
            .fold(vec![], |acc, kybra_program| {
                vec![acc, kybra_program.get_act_data_type_nodes(&dependencies)].concat()
            })
    }

    fn get_function_def_of_type(&self, method_type: CanisterMethodType) -> Vec<KybraStmt> {
        self.kybra_programs
            .iter()
            .fold(vec![], |mut acc, kybra_program| {
                acc.extend(kybra_program.get_function_defs_of_type(method_type.clone()));
                acc
            })
    }

    fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        self.kybra_programs
            .iter()
            .map(|program| program.build_stable_b_tree_map_nodes())
            .collect::<Vec<_>>()
            .concat()
    }

    fn build_external_canisters(&self) -> Vec<ActExternalCanister> {
        self.kybra_programs
            .iter()
            .map(|program| program.build_external_canisters())
            .collect::<Vec<_>>()
            .concat()
    }
}
