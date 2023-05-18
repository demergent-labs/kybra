use proc_macro2::TokenStream;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{source_map::SourceMapped, Error};

pub fn generate(
    inspect_method_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Vec<Error>> {
    let call_to_inspect_message_py_function =
        inspect_method_function_def.generate_call_to_py_function()?;

    Ok(quote::quote! {
        unsafe {
            // TODO is this a security vulnerability?
            if INTERPRETER_OPTION.is_none() {
                ic_cdk::api::call::accept_message();
                return;
            }

            let interpreter = INTERPRETER_OPTION
                .as_mut()
                .unwrap_or_trap("Unable to mutate interpreter");
            let scope = SCOPE_OPTION
                .as_mut()
                .unwrap_or_trap("Unable to mutate scope");

            interpreter.enter(|vm| {
                #call_to_inspect_message_py_function
            });
        }
    })
}
