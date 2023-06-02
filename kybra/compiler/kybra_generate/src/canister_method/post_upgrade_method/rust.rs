use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMapped;

pub fn generate(
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &String,
) -> TokenStream {
    let call_to_post_upgrade_py_function = match &post_upgrade_function_def_option {
        Some(post_upgrade_function_def) => post_upgrade_function_def
            .generate_call_to_py_function()
            .unwrap(),
        None => quote!(),
    };

    quote! {
        ic_wasi_polyfill::init(0);

        unsafe {
            let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
                // TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
                // vm.add_frozen(rustpython_pylib::frozen_stdlib());
                vm.add_native_modules(rustpython_stdlib::get_module_inits());
                vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));

                PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
                    let mut python_stdlib_stable_ref = python_stdlib_stable_ref_cell.borrow();

                    // TODO why is this necessary? It would be great to just pass in the bytes to FrozenLib::from_ref
                    let python_stdlib_bytes_reference: &'static [u8] = python_stdlib_stable_ref.get().clone().leak();

                    // let python_stdlib_bytes = python_stdlib_stable_ref.get();

                    vm.add_frozen(rustpython_compiler_core::frozen_lib::FrozenLib::from_ref(python_stdlib_bytes_reference));
                });
            });
            let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());

            _kybra_interpreter.enter(|vm| {
                Ic::make_class(&vm.ctx);
                unwrap_rust_python_result(vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm), vm);

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

            INTERPRETER_OPTION = Some(_kybra_interpreter);
            SCOPE_OPTION = Some(_kybra_scope);

            #call_to_post_upgrade_py_function

            ic_cdk_timers::set_timer(core::time::Duration::new(0, 0), rng_seed);
        }
    }
}
