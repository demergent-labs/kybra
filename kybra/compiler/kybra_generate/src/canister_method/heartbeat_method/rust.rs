use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{source_map::SourceMapped, Error};

pub fn generate(
    heartbeat_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Error> {
    let function_name = heartbeat_function_def.get_name_or_err()?;

    Ok(quote! {
        ic_cdk::spawn(async {
            call_global_python_function::<()>(#function_name, ())
                .await
                .unwrap_or_trap();
        });
    })
}
