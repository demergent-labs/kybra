use proc_macro2::TokenStream;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub fn generate(
    inspect_method_function_def: &SourceMapped<&Located<StmtKind>>,
) -> KybraResult<TokenStream> {
    let call_to_inspect_message_py_function =
        inspect_method_function_def.generate_call_to_py_function()?;

    Ok(quote::quote! {
        unsafe {
            // TODO is this a security vulnerability?
            if _KYBRA_INTERPRETER_OPTION.is_none() {
                ic_cdk::api::call::accept_message();
                return;
            }

            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

            _kybra_interpreter.enter(|vm| {
                #call_to_inspect_message_py_function
            });
        }
    })
}
