use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{source_map::SourceMapped, Error};

pub mod build;
mod rust;

pub fn generate_call(
    function_def_option: &Option<&SourceMapped<&Located<StmtKind>>>,
) -> Result<TokenStream, Vec<Error>> {
    match &function_def_option {
        Some(function_def) => function_def.generate_call_to_py_function(),
        None => Ok(quote!()),
    }
}
