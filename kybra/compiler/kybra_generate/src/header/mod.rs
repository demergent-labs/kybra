use proc_macro2::TokenStream;

mod traits;
mod use_statements;

pub fn generate() -> TokenStream {
    let use_statements = use_statements::generate();
    let traits = traits::generate();

    quote::quote! {
        #![allow(warnings, unused)]

        #use_statements
        #traits

        const PYTHON_STDLIB: &[u8] = include_bytes!("../rust_python_stdlib/stdlib");

        static mut INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;
    }
}
