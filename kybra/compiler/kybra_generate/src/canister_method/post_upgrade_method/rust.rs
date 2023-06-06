use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use crate::source_map::SourceMapped;

pub fn generate(
    init_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &String,
) -> TokenStream {
    let call_to_init_py_function = match init_function_def_option {
        Some(init_function_def) => init_function_def.generate_call_to_py_function().unwrap(),
        None => quote!(),
    };

    let call_to_post_upgrade_py_function = match &post_upgrade_function_def_option {
        Some(post_upgrade_function_def) => post_upgrade_function_def
            .generate_call_to_py_function()
            .unwrap(),
        None => quote!(),
    };

    quote! {
        let randomness = RANDOMNESS_STABLE_REF_CELL.with(|randomness_stable_ref_cell| randomness_stable_ref_cell.borrow().get().clone());

        ic_wasi_polyfill::init(u64::from_be_bytes(randomness[..8].try_into().unwrap()));

        unsafe {
            let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
                vm.add_native_modules(rustpython_stdlib::get_module_inits());
                vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));

                PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
                    let mut python_stdlib_stable_ref = python_stdlib_stable_ref_cell.borrow();

                    // TODO why is this necessary? It would be great to just pass in the bytes to FrozenLib::from_ref
                    let python_stdlib_bytes_reference: &'static [u8] = python_stdlib_stable_ref.get().clone().leak();
                    vm.add_frozen(rustpython_compiler_core::frozen_lib::FrozenLib::from_ref(python_stdlib_bytes_reference));
                });
            });
            let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());

            _kybra_interpreter.enter(|vm| {
                Ic::make_class(&vm.ctx);
                vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm).unwrap_or_trap(vm);

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

            if INITIALIZED_MAP_REF_CELL.with(|initialized_map_ref_cell| *initialized_map_ref_cell.borrow().get()) == 0 {
                #call_to_init_py_function
            }
            else {
                #call_to_post_upgrade_py_function
            }

            INITIALIZED_MAP_REF_CELL.with(|initialized_map_ref_cell| initialized_map_ref_cell.borrow_mut().set(1));
        }
    }
}
