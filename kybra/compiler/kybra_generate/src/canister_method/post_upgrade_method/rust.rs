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

    let randomness = generate_randomness();
    let interpreter_init = generate_interpreter_init();
    let ic_object_init = generate_ic_object_init();
    let code_init = generate_code_init(entry_module_name);
    let save_global_interpreter = generate_save_global_interpreter();
    let call_to_user_init_or_post_upgrade = generate_call_to_user_init_or_post_upgrade(
        &call_to_init_py_function,
        &call_to_post_upgrade_py_function,
    );

    // TODO the set_timer below is here to allow the developer to see error messages
    // TODO in their local replica during development. For some reason without
    // TODO this the error messages can't be seen, I believe because they are part of
    // TODO the cross-canister call from the kybra_deployer. We can't get that response
    // TODO currently because the Wasm binary changes before the response returns
    // TODO and calling the respone callback causes undefined behavior
    // TODO two possible solutions are coming to these issues, named/default callbacks
    // TODO and dfx chunk uploading. dfx chunk uploading should allow us to remove the
    // TODO kybra_deployer entirely, get rid of the post_install process hopefully
    // TODO and return to regular init/post_upgrade semantics
    Ok(quote! {
        ic_cdk_timers::set_timer(std::time::Duration::from_secs(0), move || {
            #randomness

            unsafe { ic_wasi_polyfill::init(&randomness); };

            #interpreter_init

            #ic_object_init

            #code_init

            #save_global_interpreter

            #call_to_user_init_or_post_upgrade
        });
    })
}

fn generate_randomness() -> TokenStream {
    quote! {
        let randomness = RANDOMNESS_STABLE_REF_CELL.with(|randomness_stable_ref_cell| randomness_stable_ref_cell.borrow().get().clone());

        if randomness.len() == 0 {
            ic_cdk::trap("Post Upgrade Error: randomness cannot have length 0");
        }
    }
}

fn generate_interpreter_init() -> TokenStream {
    quote! {
        let interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
            vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));

            PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
                let mut python_stdlib_stable_ref = python_stdlib_stable_ref_cell.borrow();

                // TODO why is this necessary? It would be great to just pass in the bytes to FrozenLib::from_ref
                let python_stdlib_bytes_reference: &'static [u8] = python_stdlib_stable_ref.get().clone().leak();
                vm.add_frozen(rustpython_compiler_core::frozen_lib::FrozenLib::from_ref(python_stdlib_bytes_reference));
            });
        });
        let scope = interpreter.enter(|vm| vm.new_scope_with_builtins());
        let vm = &interpreter.vm;
    }
}

fn generate_ic_object_init() -> TokenStream {
    quote! {
        Ic::make_class(&vm.ctx);
        vm.builtins.set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm).unwrap_or_trap(vm);
    }
}

fn generate_code_init(entry_module_name: &str) -> TokenStream {
    quote! {
        vm.run_code_string(
            scope.clone(),
            &format!("from {} import *", #entry_module_name),
            "".to_owned(),
        ).unwrap_or_trap(vm);
    }
}

fn generate_save_global_interpreter() -> TokenStream {
    quote! {
        unsafe {
            INTERPRETER_OPTION = Some(interpreter);
            SCOPE_OPTION = Some(scope);
        };
    }
}

fn generate_call_to_user_init_or_post_upgrade(
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
