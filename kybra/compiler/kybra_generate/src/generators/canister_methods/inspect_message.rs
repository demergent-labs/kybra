use proc_macro2::TokenStream;
use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMapped;

pub fn generate(inspect_method_function_def: &SourceMapped<&Located<StmtKind>>) -> TokenStream {
    let call_to_inspect_message_py_function =
        inspect_method_function_def.generate_call_to_py_function();

    quote::quote! {
        unsafe {
            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

            _kybra_interpreter.enter(|vm| {
                #call_to_inspect_message_py_function
            });
        }
    }
}
