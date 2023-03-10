use rustpython_parser::{
    ast::{Located, Mod, StmtKind},
    parser::{self, Mode},
};
use std::ops::Deref;

use crate::source_map::{SourceMap, SourceMapped};

pub struct PyAst {
    pub source_mapped_mods: Vec<SourceMapped<Mod>>,
    pub entry_module_name: String,
}

impl PyAst {
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

    pub fn get_stmt_kinds(&self) -> Vec<SourceMapped<&Located<StmtKind>>> {
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
