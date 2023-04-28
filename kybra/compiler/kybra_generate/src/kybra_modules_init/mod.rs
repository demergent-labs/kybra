// TODO this still needs a lot of cleanup and stuff

// TODO we still need to do chunk uploads

// TODO create init functions that can dynamically receive frozen python module bytecode
// TODO this should probably be more general-purpose so that we can simply upload any module and hook it into the vm
// TODO this will open the poth for numpy etc
use cdk_framework::{
    act::{
        node::{context::Context, Param},
        ToTypeAnnotation,
    },
    traits::ToIdent,
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::errors::KybraResult;

pub fn generate(
    entry_module_name: &String,
    init_params: &Vec<Param>,
    call_to_init_py_function: TokenStream,
    call_to_post_upgrade_py_function: TokenStream,
) -> KybraResult<TokenStream> {
    let params_ref_cells = init_params.iter().map(|param| {
        let init_param_name = format!("INIT_PARAM_{}", param.name).to_ident();
        let init_param_type_annotation = param.to_type_annotation(
            &Context {
                keyword_list: crate::keywords::get_python_keywords(),
                cdk_name: "kybra".to_string(),
            },
            "_kybra_init".to_string(),
        );

        quote! {
            static #init_param_name: RefCell<Option<#init_param_type_annotation>> = RefCell::new(None);
        }
    });

    let params_initializations = init_params.iter().map(|param| {
        let param_name = format!("_cdk_user_defined_{}", param.name).to_ident();
        let init_param_name = format!("INIT_PARAM_{}", param.name).to_ident();

        quote! {
            let #param_name = #init_param_name.with(|x| (*x.borrow()).clone().unwrap());
        }
    });

    Ok(quote::quote! {
        thread_local! {
            static INITIALIZED_MAP_REF_CELL: RefCell<ic_stable_structures::cell::Cell<u8, Memory>> = RefCell::new(ic_stable_structures::cell::Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(254))), 0).unwrap());

            #(#params_ref_cells)*

            // static PYTHON_SOURCE_BYTECODE_REF_CELL: RefCell<Vec<u8>> = RefCell::new(vec![]);
            // static NATIVE_STDLIB_BYTECODE_REF_CELL: RefCell<Vec<u8>> = RefCell::new(vec![]);
            // static PYTHON_STDLIB_BYTECODE_REF_CELL: RefCell<Vec<u8>> = RefCell::new(vec![]);
        }

        #[ic_cdk_macros::update]
        pub fn create_python_source_module(bytes: Vec<u8>) {
            unsafe {
                #(#params_initializations)*

                let bytes_reference: &'static [u8] = bytes.leak(); // TODO why is this necessary? It would be great to just pass in the bytes to FrozenLib::from_ref

                let _kybra_interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
                    vm.add_native_modules(rustpython_stdlib::get_module_inits());
                    vm.add_frozen(rustpython_compiler_core::frozen_lib::FrozenLib::from_ref(bytes_reference));
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

                if INITIALIZED_MAP_REF_CELL.with(|initialized_map_ref_cell| *initialized_map_ref_cell.borrow().get()) == 0 {
                    #call_to_init_py_function

                    INITIALIZED_MAP_REF_CELL.with(|initialized_map_ref_cell| initialized_map_ref_cell.borrow_mut().set(1));
                }
                else {
                    #call_to_post_upgrade_py_function
                }

                ic_cdk_timers::set_timer(core::time::Duration::new(0, 0), _cdk_rng_seed);
            }
        }
    })
}
