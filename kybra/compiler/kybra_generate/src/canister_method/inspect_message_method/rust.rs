use proc_macro2::TokenStream;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{source_map::SourceMapped, Error};

pub fn generate(
    inspect_method_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Vec<Error>> {
    let call_to_inspect_message_py_function =
        inspect_method_function_def.generate_call_to_py_function()?;

    Ok(quote::quote! {
        #call_to_inspect_message_py_function
    })
}
