mod ref_cells;
mod use_statements;

pub fn generate() -> proc_macro2::TokenStream {
    let use_statements = use_statements::generate();
    let ref_cells = ref_cells::generate();

    quote::quote! {
        #![allow(warnings, unused)]

        #use_statements

        #ref_cells

        static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut _KYBRA_SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;

        // TODO move all of this into the CDK framework once we know it works the same way in Kybra
        // Heavily inspired by https://stackoverflow.com/a/47676844
        use std::ffi::CString;
        use std::os::raw::c_char;

        #[no_mangle]
        pub fn get_candid_pointer() -> *mut c_char {
            let c_string = CString::new(__export_service()).unwrap();

            c_string.into_raw()
        }

        #[no_mangle]
        pub fn get_candid_length() -> usize {
            __export_service().len()
        }
        // TODO move all of this into the CDK framework once we know it works the same way in Kybra

        // TODO this is broken https://github.com/dfinity/motoko/issues/3462#issuecomment-1260060874
        // #[link_section = "icp:public cdk"]
        // pub static NAME: [u8; 12] = *b"kybra v0.0.0";
    }
}
