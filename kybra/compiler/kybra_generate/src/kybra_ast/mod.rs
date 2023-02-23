use cdk_framework::{
    act::{
        node::{
            canister_method::{QueryMethod, UpdateMethod},
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
        body, header,
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
    fn get_stmt_kinds(&self) -> Vec<SourceMapped<&Located<StmtKind>>> {
        self.programs.iter().fold(vec![], |acc, kybra_program| {
            let update_methods = match &kybra_program.node {
                Mod::Module { body, .. } => body
                    .iter()
                    .map(|stmt_kind| SourceMapped {
                        node: stmt_kind,
                        source_map: kybra_program.source_map.clone(),
                    })
                    .collect(),
                _ => vec![],
            };
            vec![acc, update_methods].concat()
        })
    }

    fn build_update_methods(&self) -> Vec<UpdateMethod> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_update_method())
            .collect()
    }

    fn build_query_methods(&self) -> Vec<QueryMethod> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_query_method())
            .collect()
    }

    fn build_external_canisters(&self) -> Vec<ExternalCanister> {
        vec![]
    }

    fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        vec![]
    }

    fn build_funcs(&self) -> Vec<Func> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_func())
            .collect()
    }

    fn build_records(&self) -> Vec<Record> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_record())
            .collect()
    }

    fn build_tuples(&self) -> Vec<Tuple> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_tuple())
            .collect()
    }

    fn build_type_aliases(&self) -> Vec<TypeAlias> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_type_alias())
            .collect()
    }

    fn build_variants(&self) -> Vec<Variant> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_variant())
            .collect()
    }

    fn build_guard_functions(&self) -> Vec<GuardFunction> {
        let guard_function_names = self.get_guard_function_names();
        self.get_stmt_kinds()
            .iter()
            .filter_map(|source_mapped_stmt_kind| {
                source_mapped_stmt_kind.as_guard_function(&guard_function_names)
            })
            .collect()
    }
}
