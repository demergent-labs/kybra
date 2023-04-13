use std::ops::Deref;

use cdk_framework::act::abstract_canister_tree::Error;
use proc_macro2::TokenStream;
use rustpython_parser::{
    ast::{ExprKind, Located, Mod, StmtKind},
    parser::{self, Mode},
};

use crate::{
    errors::{CreateMessage, KybraResult, Message},
    source_map::{SourceMap, SourceMapped},
};

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

    pub fn handle_cdk_framework_errors(
        &self,
        result: Result<TokenStream, Vec<Error>>,
    ) -> KybraResult<TokenStream> {
        match result {
            Ok(token_stream) => Ok(token_stream),
            Err(errors) => Err(errors
                .iter()
                .flat_map(|error| match error {
                    Error::TypeNotFound(type_ref) => self.type_not_found_error(type_ref),
                    Error::GuardFunctionNotFound(guard_function_name) => {
                        self.guard_function_not_found(guard_function_name)
                    }
                })
                .collect()),
        }
    }

    fn type_not_found_error(&self, type_ref: &str) -> Vec<Message> {
        self.find_stmts_using_type_ref(type_ref)
            .iter()
            .map(|m| {
                m.create_error_message(
                    format!("{} is used, but never defined.", type_ref).as_str(),
                    "Type ref used here",
                    None,
                )
            })
            .collect()
    }

    fn guard_function_not_found(&self, guard_function_name: &str) -> Vec<Message> {
        self.find_methods_using_guard_function_name(guard_function_name)
            .iter()
            .map(|m| {
                m.create_error_message(
                    format!("{} is used, but never defined.", guard_function_name).as_str(),
                    "Guard method used here",
                    None,
                )
            })
            .collect()
    }

    fn find_stmts_using_type_ref(&self, name: &str) -> Vec<SourceMapped<&Located<StmtKind>>> {
        self.source_mapped_mods
            .iter()
            .fold(vec![], |mut acc, program| {
                acc.extend(match &program.deref() {
                    Mod::Module { body, .. } => body
                        .iter()
                        .filter(|stmt_kind| {
                            SourceMapped::new(*stmt_kind, program.source_map.clone())
                                .uses_type_ref(name)
                        })
                        .map(|stmt_kind| SourceMapped::new(stmt_kind, program.source_map.clone()))
                        .collect(),
                    _ => vec![],
                });
                acc
            })
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn uses_type_ref(&self, name: &str) -> bool {
        false
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn uses_type_ref(&self, name: &str) -> bool {
        self.is_type_ref_named(name) || self.array_uses_type_ref(name)
    }
}
