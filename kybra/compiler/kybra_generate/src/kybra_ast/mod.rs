use cdk_framework::{
    act::{
        node::{
            canister_method::{
                CanisterMethodType, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
                QueryMethod, UpdateMethod,
            },
            data_type::{Func, Record, Tuple, TypeAlias, Variant},
            ExternalCanister, GuardFunction,
        },
        CanisterMethods, DataTypes,
    },
    AbstractCanisterTree,
};
use rustpython_parser::{
    ast::{Located, Mod, StmtKind},
    parser::{self, Mode},
};

use crate::{
    generators::{
        body,
        canister_methods::post_upgrade,
        header,
        vm_value_conversion::{try_from_vm_value_impls, try_into_vm_value_impls},
    },
    py_ast::kybra_types::StableBTreeMapNode,
    source_map::{SourceMap, SourceMapped},
};

pub mod node;

pub struct NewPyAst {
    pub programs: Vec<SourceMapped<Mod>>,
    pub entry_module_name: String,
}

impl NewPyAst {
    pub fn new(py_file_names: &Vec<&str>, entry_module_name: &str) -> NewPyAst {
        let mut mods: Vec<_> = py_file_names
            .iter()
            .enumerate()
            .map(|(_, py_file_name)| {
                let source = std::fs::read_to_string(py_file_name).unwrap();

                parser::parse(&source, Mode::Module, "").unwrap()
            })
            .collect();

        NewPyAst {
            programs: mods
                .drain(..)
                .enumerate()
                .map(|(index, my_mod)| {
                    let source = std::fs::read_to_string(py_file_names[index]).unwrap();

                    SourceMapped {
                        source_map: SourceMap::new(source.clone(), py_file_names[index]),
                        node: my_mod,
                    }
                })
                .collect(),
            entry_module_name: entry_module_name.to_string(),
        }
    }
}

impl NewPyAst {
    pub fn to_act(&self) -> AbstractCanisterTree {
        let stable_b_tree_map_nodes = self.build_stable_b_tree_map_nodes();
        let external_canisters = self.build_external_canisters();

        let canister_methods = CanisterMethods {
            heartbeat_method: self.build_heartbeat_method(),
            init_method: self.build_init_method(),
            inspect_message_method: self.build_inspect_method(),
            post_upgrade_method: self.build_post_upgrade_method(),
            pre_upgrade_method: self.build_pre_upgrade_method(),
            query_methods: self.build_query_methods(),
            update_methods: self.build_update_methods(),
        };

        let data_types = DataTypes {
            funcs: self.build_funcs(),
            records: self.build_records(),
            tuples: self.build_tuples(),
            type_aliases: self.build_type_aliases(),
            variants: self.build_variants(),
        };

        AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            header: header::generate(),
            body: body::generate(
                &canister_methods.update_methods,
                &canister_methods.query_methods,
                &external_canisters,
                &stable_b_tree_map_nodes,
            ),
            data_types,
            canister_methods,
            external_canisters,
            guard_functions: self.build_guard_functions(),
            try_from_vm_value_impls: try_into_vm_value_impls::generate(),
            try_into_vm_value_impls: try_from_vm_value_impls::generate(),
            keywords: crate::get_python_keywords(),
        }
    }
}

impl NewPyAst {
    fn build_update_methods(&self) -> Vec<UpdateMethod> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let update_methods = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_update_method()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, update_methods].concat()
        })
    }

    fn build_query_methods(&self) -> Vec<QueryMethod> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let query_methods = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_query_method()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, query_methods].concat()
        })
    }

    fn build_external_canisters(&self) -> Vec<ExternalCanister> {
        vec![]
    }

    fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        vec![]
    }

    fn build_inspect_method(&self) -> Option<InspectMessageMethod> {
        None
    }

    fn build_post_upgrade_method(&self) -> PostUpgradeMethod {
        let post_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => post_upgrade_function_def.build_params(),
            None => vec![],
        };

        let body = post_upgrade::generate_post_upgrade_method_body_cdk_refactor_name(
            post_upgrade_function_def_option,
            &self.entry_module_name,
        );

        PostUpgradeMethod { params, body }
    }

    fn build_pre_upgrade_method(&self) -> Option<PreUpgradeMethod> {
        None
    }

    fn build_funcs(&self) -> Vec<Func> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let funcs = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_func()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, funcs].concat()
        })
    }

    fn build_records(&self) -> Vec<Record> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let records = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_record()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, records].concat()
        })
    }

    fn build_tuples(&self) -> Vec<Tuple> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let tuples = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_tuple()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, tuples].concat()
        })
    }

    fn build_type_aliases(&self) -> Vec<TypeAlias> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let type_aliases = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_type_alias()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, type_aliases].concat()
        })
    }

    fn build_variants(&self) -> Vec<Variant> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let variants = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter_map(|stmt_kind| {
                        SourceMapped {
                            node: stmt_kind,
                            source_map: kybra_program.source_map.clone(),
                        }
                        .as_variant()
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, variants].concat()
        })
    }

    fn build_guard_functions(&self) -> Vec<GuardFunction> {
        vec![]
    }
}

impl NewPyAst {
    fn get_function_def_of_type(
        &self,
        method_type: CanisterMethodType,
    ) -> Vec<SourceMapped<&Located<StmtKind>>> {
        self.programs.iter().fold(vec![], |mut acc, program| {
            let thing = match &program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .filter(|stmt_kind| {
                        SourceMapped {
                            node: *stmt_kind,
                            source_map: program.source_map.clone(),
                        }
                        .is_canister_method_type(method_type.clone())
                    })
                    .map(|stmt_kind| SourceMapped {
                        node: stmt_kind,
                        source_map: program.source_map.clone(),
                    })
                    .collect(),
                _ => vec![],
            };
            acc.extend(thing);
            acc
        })
    }
}
