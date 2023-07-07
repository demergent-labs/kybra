use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{source_map::SourceMapped, Error};

pub fn generate(
    pre_upgrade_function_def: &SourceMapped<&Located<StmtKind>>,
) -> Result<TokenStream, Vec<Error>> {
    let call_to_pre_upgrade_py_function =
        pre_upgrade_function_def.generate_call_to_py_function()?;

    Ok(quote! {
        if unsafe { INTERPRETER_OPTION.as_mut() }.is_none() {
            ic_cdk::println!("Pre Upgrade Warning: The interpreter is not defined. The previous init or post_upgrade has most likely trapped");
            return;
        }

        #call_to_pre_upgrade_py_function
    })
}
