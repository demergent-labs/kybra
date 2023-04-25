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
            let vm_interpreter = VM_INTERPRETER_OPTION.as_mut().unwrap();
            let vm_scope = VM_SCOPE.as_mut().unwrap();

            vm_interpreter.enter(|vm| {
                #call_to_pre_upgrade_py_function
            });
        }
    })
}
