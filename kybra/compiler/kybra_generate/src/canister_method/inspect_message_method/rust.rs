use proc_macro2::TokenStream;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub fn generate(
    inspect_method_function_def: &SourceMapped<&Located<StmtKind>>,
) -> KybraResult<TokenStream> {
    let call_to_inspect_message_py_function =
        inspect_method_function_def.generate_call_to_py_function()?;

    Ok(quote::quote! {
        // TODO perhaps we can do some authentication here? Probably better elsewhere
        // TODO is this a security vulnerability?
        if ic_cdk::api::call::method_name() == "create_python_source_module" {
            ic_cdk::api::call::accept_message();
            return;
        }

        unsafe {
            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

            _kybra_interpreter.enter(|vm| {
                #call_to_inspect_message_py_function
            });
        }
    })
}
