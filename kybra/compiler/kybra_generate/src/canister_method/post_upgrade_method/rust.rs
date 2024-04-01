// TODO move the logic into init and import it into post_upgrade

use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use super::generate_call;
use crate::{source_map::SourceMapped, Error};

pub fn generate(
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &String,
) -> Result<TokenStream, Vec<Error>> {
    let call_to_post_upgrade_py_function = generate_call(&post_upgrade_function_def_option)?;

    let interpreter_init = generate_interpreter_init();
    let ic_object_init = generate_ic_object_init();
    let code_init = generate_code_init(entry_module_name);
    let save_global_interpreter = generate_save_global_interpreter();
    let randomness = generate_randomness();

    Ok(quote! {
        ic_wasi_polyfill::init(&[], &[]);

        #interpreter_init

        #ic_object_init

        #code_init

        #save_global_interpreter

        // This block is to avoid some difficult typings in call_global_python_function_sync
        // If you make call_global_python_function_sync not return immediately by ommitting the semi-colon
        // then you must explicitly type it which has proved somewhat difficult
        {
            // This is here so that the py function call below has access to the vm
            // The vm ownership is transferred above, thus we do this for now
            let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }.unwrap_or_trap("SystemError: missing python interpreter");
            let vm = &interpreter.vm;

            #call_to_post_upgrade_py_function
        }

        #randomness
    })
}

pub fn generate_interpreter_init() -> TokenStream {
    quote! {
        let interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
            ic_cdk::println!("with_init 0");

            vm.add_native_modules(rustpython_stdlib::get_module_inits());

            ic_cdk::println!("with_init 1");

            vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));

            ic_cdk::println!("with_init 2");

            PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {

                ic_cdk::println!("with_init 3");

                #[cfg(feature = "azle_include_stdlib")]
                {
                    ic_cdk::println!("with_init 4");

                    // TODO can we simplify this a bit?
                    // TODO make sure to use a dynamic path here
                    // TODO we will have to somehow pass in the current kybra version
                    let mut python_stdlib_stable_ref_mut = python_stdlib_stable_ref_cell.borrow_mut();

                    ic_cdk::println!("with_init 5");

                    python_stdlib_stable_ref_mut
                        .set(rustpython_vm::py_freeze!(dir = "Lib").bytes.to_vec())
                        .unwrap();
                        // .map_err(|err| value_error_to_string(&err));

                    ic_cdk::println!("with_init 6");
                }

                ic_cdk::println!("with_init 7");

                // TODO why is this necessary? It would be great to just pass in the bytes to FrozenLib::from_ref
                let python_stdlib_bytes_reference: &'static [u8] = python_stdlib_stable_ref_cell.borrow().get().clone().leak();

                ic_cdk::println!("with_init 8");

                ic_cdk::println!("length: {}", python_stdlib_bytes_reference.len());

                vm.add_frozen(rustpython_compiler_core::frozen_lib::FrozenLib::from_ref(python_stdlib_bytes_reference));

                ic_cdk::println!("with_init 9");

                pub fn value_error_to_string(err: &ic_stable_structures::cell::ValueError) -> String {
                    match err {
                        ic_stable_structures::cell::ValueError::ValueTooLarge { value_size } => {
                            format!("ValueError: ValueTooLarge {value_size}")
                        }
                    }
                }
            });
        });
        let scope = interpreter.enter(|vm| vm.new_scope_with_builtins());
        let vm = &interpreter.vm;
    }
}

pub fn generate_ic_object_init() -> TokenStream {
    quote! {
        Ic::make_class(&vm.ctx);
        vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm).unwrap_or_trap(vm);
    }
}

pub fn generate_code_init(entry_module_name: &str) -> TokenStream {
    quote! {
        vm.run_code_string(
            scope.clone(),
            &format!("from {} import *", #entry_module_name),
            "".to_owned(),
        ).unwrap_or_trap(vm);
    }
}

pub fn generate_save_global_interpreter() -> TokenStream {
    quote! {
        unsafe {
            INTERPRETER_OPTION = Some(interpreter);
            SCOPE_OPTION = Some(scope);
        };
    }
}

pub fn generate_randomness() -> TokenStream {
    quote! {
        ic_cdk_timers::set_timer(std::time::Duration::from_secs(0), || {
            ic_cdk::spawn(async move {
                let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::management_canister::main::raw_rand().await;

                match result {
                    Ok(randomness) => {
                        let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
                            .ok_or_else(|| "SystemError: missing python interpreter".to_string()).unwrap();
                        let scope = unsafe { SCOPE_OPTION.as_mut() }
                            .ok_or_else(|| "SystemError: missing python scope".to_string()).unwrap();

                        interpreter.enter(|vm| {
                            let random_module = vm.import("random", None, 0).unwrap();
                            let seed_fn = random_module.get_attr("seed", vm).unwrap();

                            seed_fn.call((vm.ctx.new_bytes(randomness.0),), vm).unwrap();
                        });
                    },
                    Err(err) => panic!(err)
                };
            });
        });
    }
}
