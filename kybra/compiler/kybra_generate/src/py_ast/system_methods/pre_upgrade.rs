use quote::quote;

use crate::{
    cdk_act::{nodes::ActPreUpgradeMethod, CanisterMethodType},
    py_ast::PyAst,
};

impl PyAst<'_> {
    pub fn build_pre_upgrade_method(&self) -> ActPreUpgradeMethod {
        let pre_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PreUpgrade);

        if pre_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let call_to_pre_upgrade_py_function = match pre_upgrade_function_defs.get(0) {
            Some(pre_upgrade_function_def) => {
                pre_upgrade_function_def.generate_call_to_py_function()
            }
            None => quote!(),
        };

        let body = quote! {
            let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
            let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

            _kybra_interpreter.enter(|vm| {
                #call_to_pre_upgrade_py_function
            });

            // TODO handle stable storage stuff

        };

        ActPreUpgradeMethod { body }
    }
}
