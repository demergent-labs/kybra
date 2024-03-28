// TODO move the logic into init and import it into post_upgrade

use cdk_framework::traits::CollectResults;
use proc_macro2::TokenStream;
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};

use super::generate_call;
use crate::{source_map::SourceMapped, Error};

pub fn generate(
    init_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    post_upgrade_function_def_option: Option<&SourceMapped<&Located<StmtKind>>>,
    entry_module_name: &String,
) -> Result<TokenStream, Vec<Error>> {
    let (call_to_init_py_function, call_to_post_upgrade_py_function) = (
        generate_call(&init_function_def_option),
        generate_call(&post_upgrade_function_def_option),
    )
        .collect_results()?;

    let interpreter_init = generate_interpreter_init();
    let ic_object_init = generate_ic_object_init();
    let code_init = generate_code_init(entry_module_name);
    let save_global_interpreter = generate_save_global_interpreter();
    let call_to_user_init_or_post_upgrade = generate_call_to_user_init_or_post_upgrade(
        &call_to_init_py_function,
        &call_to_post_upgrade_py_function,
    );

    Ok(quote! {
        unsafe { ic_wasi_polyfill::init(&[], &[]); };

        #interpreter_init

        #ic_object_init

        #code_init

        #save_global_interpreter

        #call_to_user_init_or_post_upgrade
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

pub fn generate_call_to_user_init_or_post_upgrade(
    call_to_init_py_function: &TokenStream,
    call_to_post_upgrade_py_function: &TokenStream,
) -> TokenStream {
    quote! {
            // This is here so that the py function calls below have access to the vm
            // The vm ownership is transferred above, thus we do this for now
            let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }.unwrap_or_trap("SystemError: missing python interpreter");
            let vm = &interpreter.vm;

            if CANISTER_INITIALIZED_REF_CELL.with(|canister_initialized_ref_cell| *canister_initialized_ref_cell.borrow().get()) == 0 {
                #call_to_init_py_function
            }
            else {
                #call_to_post_upgrade_py_function
            }

            CANISTER_INITIALIZED_REF_CELL.with(|canister_initialized_ref_cell| canister_initialized_ref_cell.borrow_mut().set(1).unwrap_or_trap());
    }
}
