use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::canister_method::post_upgrade_method::generate_call;
use crate::canister_method::post_upgrade_method::rust::{
    generate_code_init, generate_ic_object_init, generate_interpreter_init, generate_randomness,
    generate_save_global_interpreter,
};
use crate::{source_map::SourceMapped, Error};

pub fn generate(
    init_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<TokenStream, Vec<Error>> {
    let call_to_init_py_function = generate_call(&init_function_def_option)?;

    let interpreter_init = generate_interpreter_init();
    let ic_object_init = generate_ic_object_init();
    let code_init = generate_code_init(entry_module_name);
    let save_global_interpreter = generate_save_global_interpreter();
    let randomness = generate_randomness();

    Ok(quote! {
        ic_wasi_polyfill::init(&[], &[]);

        #interpreter_init

        #ic_object_init

        #code_init

        #save_global_interpreter

        // This block is to avoid some difficult typings in call_global_python_function_sync
        // If you make call_global_python_function_sync not return immediately by ommitting the semi-colon
        // then you must explicitly type it which has proved somewhat difficult
        {
            #call_to_init_py_function
        }

        #randomness
    })
}
