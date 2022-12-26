use cdk_framework::{ToAct, ToTokenStream};
use py_ast::{KybraProgram, PyAst};
use quote::quote;
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

pub fn kybra_generate(
    py_file_names: &Vec<&str>,
    entry_module_name: &str,
) -> proc_macro2::token_stream::TokenStream {
    eprintln!("-------------------------------------------");
    eprintln!("--- STARTING ------------------------------");
    eprintln!("-------------------------------------------");

    let kybra_programs: Vec<KybraProgram> = py_file_names
        .iter()
        .map(|py_file_name| {
            let source = std::fs::read_to_string(py_file_name).unwrap();

            let source_map = SourceMap::new(&source, py_file_name);

            KybraProgram {
                program: parser::parse(&source, Mode::Module, "").unwrap(),
                source_map,
            }
        })
        .collect();
    let canister_definition = PyAst {
        kybra_programs,
        entry_module_name: entry_module_name.to_string(),
    }
    .analyze()
    .to_kybra_ast()
    .to_act()
    .to_token_stream(());
    eprintln!("-------------------------------------------");
    eprintln!("--- ENDING --------------------------------");
    eprintln!("-------------------------------------------");

    quote! {
        #![allow(warnings, unused)]

        use rustpython_vm::{AsObject, builtins::{PyDict, PyBaseException, PyGenerator, PyListRef, PyTupleRef, PyIntRef, PyStr, PyList, PyTuple, PyBytes}, class::PyClassImpl, convert::ToPyObject, function::IntoFuncArgs, PyObjectRef, PyObject, PyRef, VirtualMachine, protocol::{PyIter, PyIterReturn}, py_serde::{deserialize, serialize}};
        use rustpython_derive::{pyclass, PyPayload};
        use kybra_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
        use std::str::FromStr;
        use ic_cdk::api::call::CallResult;
        use serde::de::{DeserializeSeed, Visitor};
        use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeTuple};
        use slotmap::Key;

        static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut _KYBRA_SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;

        // TODO this is broken https://github.com/dfinity/motoko/issues/3462#issuecomment-1260060874
        // #[link_section = "icp:public cdk"]
        // pub static NAME: [u8; 12] = *b"kybra v0.0.0";

        #canister_definition

    }
}
