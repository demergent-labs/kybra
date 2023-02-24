use kybra_ast::NewPyAst;
use proc_macro2::TokenStream;

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
    NewPyAst::new(py_file_names, entry_module_name)
        .to_act()
        .to_token_stream()
}
