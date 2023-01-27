use quote::quote;

use crate::py_ast::kybra_types::KybraStmt;

pub fn generate_pre_upgrade_method_body(
    pre_upgrade_function_def_option: Option<&KybraStmt>,
) -> proc_macro2::TokenStream {
    let call_to_pre_upgrade_py_function = match pre_upgrade_function_def_option {
        Some(pre_upgrade_function_def) => pre_upgrade_function_def.generate_call_to_py_function(),
        None => quote!(),
    };

    quote! {
        unsafe {
            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

            _kybra_interpreter.enter(|vm| {
                #call_to_pre_upgrade_py_function
            });
        }
    }
}