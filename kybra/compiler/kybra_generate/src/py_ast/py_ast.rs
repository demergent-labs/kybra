use rustpython_parser::{
    ast::Mod,
    parser::{self, Mode},
};

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
}
