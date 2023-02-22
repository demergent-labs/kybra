use cdk_framework::act::ToAct;
use kybra_ast::NewPyAst;
use proc_macro2::TokenStream;
use py_ast::{kybra_types::KybraProgram, PyAst};
use rustpython_parser::parser::{self, Mode};
use source_map::{SourceMap, SourceMapped};

mod errors;
mod generators;
pub mod kybra_ast;
pub mod py_ast;
pub mod source_map;

const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

pub fn get_python_keywords() -> Vec<String> {
    PYTHON_KEYWORDS
        .iter()
        .map(|keyword| keyword.to_string())
        .collect()
}

pub fn generate_canister(py_file_names: &Vec<&str>, entry_module_name: &str) -> TokenStream {
    let kybra_programs: Vec<KybraProgram> = py_file_names
        .iter()
        .enumerate()
        .map(|(_, py_file_name)| {
            let source = std::fs::read_to_string(py_file_name).unwrap();

            let source_map = SourceMap::new(source.clone(), py_file_name);
            KybraProgram {
                program: parser::parse(&source, Mode::Module, "").unwrap(),
                source_map: source_map.clone(),
            }
        })
        .collect();
    let mods: Vec<_> = py_file_names
        .iter()
        .enumerate()
        .map(|(_, py_file_name)| {
            let source = std::fs::read_to_string(py_file_name).unwrap();

            parser::parse(&source, Mode::Module, "").unwrap()
        })
        .collect();

    let _old = PyAst {
        kybra_programs,
        entry_module_name: entry_module_name.to_string(),
    }
    .analyze()
    .to_kybra_ast()
    .to_act()
    .to_token_stream();

    let programs: Vec<_> = py_file_names
        .iter()
        .enumerate()
        .map(|(index, py_file_name)| {
            let source = std::fs::read_to_string(py_file_name).unwrap();

            SourceMapped {
                source_map: SourceMap::new(source.clone(), py_file_name),
                node: &mods[index],
            }
        })
        .collect();
    NewPyAst {
        programs,
        entry_module_name: entry_module_name.to_string(),
    }
    .to_act()
    .to_token_stream()
}
