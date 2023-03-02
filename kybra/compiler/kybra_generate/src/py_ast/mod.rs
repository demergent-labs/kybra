use std::ops::Deref;

use cdk_framework::{
    act::{CanisterMethods, DataTypes},
    AbstractCanisterTree,
};
use rustpython_parser::{
    ast::{Located, Mod, StmtKind},
    parser::{self, Mode},
};

use crate::{
    errors::KybraResult,
    generators::{
        body, header,
        vm_value_conversion::{try_from_vm_value_impls, try_into_vm_value_impls},
    },
    source_map::{SourceMap, SourceMapped},
};

pub mod debug;
pub mod node;

pub struct PyAst {
    pub source_mapped_mods: Vec<SourceMapped<Mod>>,
    pub entry_module_name: String,
}

impl PyAst {
    pub fn to_act(&self) -> KybraResult<AbstractCanisterTree> {
        let stable_b_tree_map_nodes = self.build_stable_b_tree_map_nodes();
        let external_canisters = self.build_external_canisters();

        let canister_methods = CanisterMethods {
            heartbeat_method: self.build_heartbeat_method()?,
            init_method: self.build_init_method()?,
            inspect_message_method: self.build_inspect_method()?,
            post_upgrade_method: self.build_post_upgrade_method()?,
            pre_upgrade_method: self.build_pre_upgrade_method()?,
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

        Ok(AbstractCanisterTree {
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
        })
    }

    pub fn new(py_file_names: &Vec<&str>, entry_module_name: &str) -> PyAst {
        let mut mods: Vec<_> = py_file_names
            .iter()
            .enumerate()
            .map(|(_, py_file_name)| {
                let source = std::fs::read_to_string(py_file_name).unwrap();

                parser::parse(&source, Mode::Module, "").unwrap()
            })
            .collect();

        PyAst {
            source_mapped_mods: mods
                .drain(..)
                .enumerate()
                .map(|(index, my_mod)| {
                    let source = std::fs::read_to_string(py_file_names[index]).unwrap();

                    SourceMapped::new(my_mod, SourceMap::new(source.clone(), py_file_names[index]))
                })
                .collect(),
            entry_module_name: entry_module_name.to_string(),
        }
    }

    fn get_stmt_kinds(&self) -> Vec<SourceMapped<&Located<StmtKind>>> {
        self.source_mapped_mods
            .iter()
            .fold(vec![], |acc, source_mapped_mod| {
                let source_mapped_stmt_kinds = match &source_mapped_mod.deref() {
                    Mod::Module { body, .. } => body
                        .iter()
                        .map(|stmt_kind| {
                            SourceMapped::new(stmt_kind, source_mapped_mod.source_map.clone())
                        })
                        .collect(),
                    _ => vec![],
                };
                vec![acc, source_mapped_stmt_kinds].concat()
            })
    }
}
