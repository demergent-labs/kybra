use proc_macro2::TokenStream;
use quote::quote;

use crate::py_ast::kybra_types::KybraStmt;

pub fn generate_post_upgrade_method_body(
    post_upgrade_function_def_option: Option<&KybraStmt>,
    entry_module_name: &String,
) -> TokenStream {
    let call_to_post_upgrade_py_function = match &post_upgrade_function_def_option {
        Some(post_upgrade_function_def) => post_upgrade_function_def.generate_call_to_py_function(),
        None => quote!(),
    };

    quote! {
        unsafe {
            let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
                // TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
                // vm.add_frozen(rustpython_pylib::frozen_stdlib());
                vm.add_native_modules(rustpython_stdlib::get_module_inits());
                vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));
            });
            let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());

            _kybra_interpreter.enter(|vm| {
                Ic::make_class(&vm.ctx);
                _kybra_unwrap_rust_python_result(vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm), vm);

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

            #call_to_post_upgrade_py_function

            ic_cdk::timer::set_timer(core::time::Duration::new(0, 0), _azle_rng_seed);
        }
    }
}
