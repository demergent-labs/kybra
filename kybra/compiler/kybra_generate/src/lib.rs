use cdk_framework::{ToAct, ToTokenStream};
use py_ast::{KybraProgram, PyAst};
use quote::quote;
use rustpython_parser::parser::{self, Mode};
use source_map::SourceMap;

mod errors;
mod generators;
mod py_ast;
mod source_map;

pub fn kybra_generate(
    py_file_names: &Vec<&str>,
    entry_module_name: &str,
) -> proc_macro2::token_stream::TokenStream {
    eprintln!("-------------------------------------------");
    eprintln!("--- STARTING ------------------------------");
    eprintln!("-------------------------------------------");

    let source_map = SourceMap {};
    let kybra_programs: Vec<KybraProgram> = py_file_names
        .iter()
        .map(|py_file_name| {
            let source = std::fs::read_to_string(py_file_name).unwrap();

            KybraProgram {
                program: parser::parse(&source, Mode::Module, "").unwrap(),
                source_map: &source_map,
            }
        })
        .collect();
    let canister_definition = PyAst {
        kybra_programs,
        entry_module_name: entry_module_name.to_string(),
    }
    .to_kybra_ast()
    .to_act()
    .to_token_stream(());
    eprintln!("-------------------------------------------");
    eprintln!("--- ENDING --------------------------------");
    eprintln!("-------------------------------------------");

    quote! {
        use rustpython_vm::{AsObject, builtins::{PyBaseException, PyGenerator, PyListRef, PyTupleRef, PyIntRef}, class::PyClassImpl, convert::ToPyObject, function::IntoFuncArgs, PyObjectRef, PyRef, VirtualMachine, protocol::{PyIter, PyIterReturn}, py_serde::{deserialize, serialize}};
        use rustpython_derive::{pyclass, PyPayload};
        use kybra_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
        use std::str::FromStr;
        use ic_cdk::api::call::CallResult;

        static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut _KYBRA_SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;

        // TODO this is broken https://github.com/dfinity/motoko/issues/3462#issuecomment-1260060874
        // #[link_section = "icp:public cdk"]
        // pub static NAME: [u8; 12] = *b"kybra v0.0.0";

        #canister_definition

    }
}
