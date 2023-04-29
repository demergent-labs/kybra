use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub fn generate(
    pre_upgrade_function_def: &SourceMapped<&Located<StmtKind>>,
) -> KybraResult<TokenStream> {
    let call_to_pre_upgrade_py_function =
        pre_upgrade_function_def.generate_call_to_py_function()?;

    Ok(quote! {
        unsafe {
            let interpreter = INTERPRETER_OPTION.as_mut().unwrap();
            let scope = SCOPE_OPTION.as_mut().unwrap();

            interpreter.enter(|vm| {
                #call_to_pre_upgrade_py_function
            });
        }
    })
}
