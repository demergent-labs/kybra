use cdk_framework::ToAct;
use py_ast::{KybraProgram, PyAst};
use rustpython_parser::parser::{self, Mode};
use source_map::SourceMap;

mod errors;
mod generators;
mod py_ast;
mod source_map;

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

pub fn generate_canister(
    py_file_names: &Vec<&str>,
    entry_module_name: &str,
) -> proc_macro2::token_stream::TokenStream {
    let source_maps: Vec<_> = py_file_names
        .iter()
        .map(|py_file_name| SourceMap {
            file_name: py_file_name.to_string(),
        })
        .collect();
    let kybra_programs: Vec<KybraProgram> = py_file_names
        .iter()
        .enumerate()
        .map(|(index, py_file_name)| {
            let source = std::fs::read_to_string(py_file_name).unwrap();

            KybraProgram {
                program: parser::parse(&source, Mode::Module, "").unwrap(),
                source_map: &source_maps[index],
            }
        })
        .collect();

    PyAst {
        kybra_programs,
        entry_module_name: entry_module_name.to_string(),
    }
    .to_kybra_ast()
    .to_act()
    .to_token_stream()
}
