use quote::quote;

use crate::{generators::ic_object, py_ast::PyAst};
use cdk_framework::{nodes::ActPostUpgradeMethod, CanisterMethodType};

impl PyAst<'_> {
    pub fn build_post_upgrade_method(&self) -> ActPostUpgradeMethod {
        let ic_object = ic_object::generate_ic_object();

        let post_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PreUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => post_upgrade_function_def.build_params(),
            None => vec![],
        };

        let call_to_post_upgrade_py_function = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => {
                post_upgrade_function_def.generate_call_to_py_function()
            }
            None => quote!(),
        };

        let entry_module_name = &self.entry_module_name;
        let body = quote::quote! {
            unsafe {
                let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
                    vm.add_native_modules(rustpython_stdlib::get_module_inits());
                    vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));
                });
                let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());

                _kybra_interpreter.enter(|vm| {
                    Ic::make_class(&vm.ctx);
                    vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm);

                    let result = vm.run_code_string(
                        _kybra_scope.clone(),
                        &format!("from {} import *", #entry_module_name),
                        "".to_owned(),
                    );

                    if let Err(err) = result {
                        let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                        panic!("{}", err_string);
                    }
                });

                _KYBRA_INTERPRETER_OPTION = Some(_kybra_interpreter);
                _KYBRA_SCOPE_OPTION = Some(_kybra_scope);

                #ic_object

                #call_to_post_upgrade_py_function
            }
        };
        ActPostUpgradeMethod { params, body }
    }
}
