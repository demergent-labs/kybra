use proc_macro2::TokenStream;

mod ref_cells;
mod use_statements;

pub fn generate() -> TokenStream {
    let use_statements = use_statements::generate();
    let ref_cells = ref_cells::generate();

    quote::quote! {
        #![allow(warnings, unused)]

        #use_statements

        #ref_cells

        static mut VM_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut VM_SCOPE: Option<rustpython_vm::scope::Scope> = None;

        // TODO this is broken https://github.com/dfinity/motoko/issues/3462#issuecomment-1260060874
        // #[link_section = "icp:public cdk"]
        // pub static NAME: [u8; 12] = *b"kybra v0.0.0";
    }
}
