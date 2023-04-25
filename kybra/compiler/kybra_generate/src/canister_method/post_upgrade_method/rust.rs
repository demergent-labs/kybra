use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::KybraResult, source_map::SourceMapped};

pub fn generate(
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &String,
) -> KybraResult<TokenStream> {
    let call_to_post_upgrade_py_function = match &post_upgrade_function_def_option {
        Some(post_upgrade_function_def) => {
            post_upgrade_function_def.generate_call_to_py_function()?
        }
        None => quote!(),
    };

    Ok(quote! {
        unsafe {
            let vm_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
                // TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
                // vm.add_frozen(rustpython_pylib::frozen_stdlib());
                vm.add_native_modules(rustpython_stdlib::get_module_inits());
                vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));
            });
            let vm_scope = vm_interpreter.enter(|vm| vm.new_scope_with_builtins());

            vm_interpreter.enter(|vm| {
                Ic::make_class(&vm.ctx);
                unwrap_rust_python_result(vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm), vm);

                let result = vm.run_code_string(
                    vm_scope.clone(),
                    &format!("from {} import *", #entry_module_name),
                    "".to_owned(),
                );

                if let Err(err) = result {
                    let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                    panic!("{}", err_string);
                }
            });

            VM_INTERPRETER_OPTION = Some(vm_interpreter);
            VM_SCOPE = Some(vm_scope);

            #call_to_post_upgrade_py_function

            ic_cdk_timers::set_timer(core::time::Duration::new(0, 0), rng_seed);
        }
    })
}
