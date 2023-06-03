use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{source_map::SourceMapped, Error};

pub fn generate(
    heartbeat_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Error> {
    let function_name = heartbeat_function_def.get_name_or_err()?;

    Ok(quote! {
        unsafe {
            ic_cdk::spawn(async {
                let interpreter = INTERPRETER_OPTION
                    .as_mut()
                    .unwrap_or_trap("SystemError: missing python interpreter");
                let scope = SCOPE_OPTION
                    .as_mut()
                    .unwrap_or_trap("SystemError: missing python scope");

                let vm = &interpreter.vm;

                let py_object_ref = scope
                    .globals
                    .get_item(#function_name, vm).unwrap_or_trap(vm)
                    .call((), vm).unwrap_or_trap(vm);

                async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
            });
        }
    })
}
