use cdk_framework::traits::CollectResults;
use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::canister_method::post_upgrade_method::generate_call;
use crate::canister_method::post_upgrade_method::rust::{
    generate_call_to_user_init_or_post_upgrade, generate_code_init, generate_ic_object_init,
    generate_interpreter_init, generate_save_global_interpreter,
};
use crate::{source_map::SourceMapped, Error};

pub fn generate(
    init_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &str,
) -> Result<TokenStream, Vec<Error>> {
    let (call_to_init_py_function, call_to_post_upgrade_py_function) = (
        generate_call(&init_function_def_option),
        generate_call(&post_upgrade_function_def_option),
    )
        .collect_results()?;

    let interpreter_init = generate_interpreter_init();
    let ic_object_init = generate_ic_object_init();
    let code_init = generate_code_init(entry_module_name);
    let save_global_interpreter = generate_save_global_interpreter();
    let call_to_user_init_or_post_upgrade = generate_call_to_user_init_or_post_upgrade(
        &call_to_init_py_function,
        &call_to_post_upgrade_py_function,
    );

    Ok(quote! {
        unsafe { ic_wasi_polyfill::init(&[], &[]); };

        #interpreter_init

        #ic_object_init

        #code_init

        #save_global_interpreter

        #call_to_user_init_or_post_upgrade
    })
}
