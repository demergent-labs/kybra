use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    canister_method::init_method::rust::{
        generate_call, generate_call_to_py_function_with_interpreter_and_vm, generate_code_init,
        generate_ic_object_init, generate_interpreter_init, generate_randomness,
        generate_save_global_interpreter,
    },
    source_map::SourceMapped,
    Error,
};

pub fn generate(
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<TokenStream, Vec<Error>> {
    let interpreter_init = generate_interpreter_init();
    let ic_object_init = generate_ic_object_init();
    let code_init = generate_code_init(entry_module_name);
    let save_global_interpreter = generate_save_global_interpreter();
    let call_to_post_upgrade_py_function = generate_call_to_py_function_with_interpreter_and_vm(
        &generate_call(&post_upgrade_function_def_option)?,
    );
    let randomness = generate_randomness();

    Ok(quote! {
        ic_wasi_polyfill::init(&[], &[]);

        #interpreter_init

        #ic_object_init

        #code_init

        #save_global_interpreter

        #call_to_post_upgrade_py_function

        #randomness
    })
}
