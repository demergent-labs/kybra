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
            let vm_interpreter = VM_INTERPRETER_OPTION.as_mut().unwrap();
            let vm_scope = VM_SCOPE.as_mut().unwrap();

            vm_interpreter.enter(|vm| {
                #call_to_inspect_message_py_function
            });
        }
    })
}
