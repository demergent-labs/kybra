use cdk_act::generators::{
    try_from_vm_value::generate_try_from_vm_value, try_into_vm_value::generate_try_into_vm_value,
};
use generators::{
    try_from_vm_value_impl::generate_try_from_vm_value_impl,
    try_into_vm_value_impl::generate_try_into_vm_value_impl,
};
use quote::quote;

mod cdk_act;
mod generators;

pub fn kybra_generate(main_py: &str) -> proc_macro2::token_stream::TokenStream {
    // TODO keep it simple
    // TODO go get the functions and create the function bodies
    // TODO do everything with just strings
    // TODO try to follow the py_ast, cdk_act path that azle is following
    // TODO try not to do work that cdk_act will do for us

    // TODO the absolute first step is to get a python AST and walk it to get the functions
    // TODO then we determine if those functions are query or update functions
    // TODO then we create those functions' token streams

    let try_into_vm_value = generate_try_into_vm_value();
    let try_into_vm_value_impl = generate_try_into_vm_value_impl();

    let try_from_vm_value = generate_try_from_vm_value();
    let try_from_vm_value_impl = generate_try_from_vm_value_impl();

    quote! {
        use rustpython;
        use rustpython::vm::convert::ToPyObject;

        static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython::vm::Interpreter> = None;
        static mut _KYBRA_SCOPE_OPTION: Option<rustpython::vm::scope::Scope> = None;

        static MAIN_PY: &'static str = #main_py;

        fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
            Ok(())
        }

        getrandom::register_custom_getrandom!(custom_getrandom);

        #try_into_vm_value
        #try_into_vm_value_impl

        #try_from_vm_value
        #try_from_vm_value_impl

        #[ic_cdk_macros::init]
        fn _kybra_init() {
            unsafe {
                let _kybra_interpreter = rustpython::vm::Interpreter::without_stdlib(Default::default());
                let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());

                _kybra_interpreter.enter(|vm| {
                    vm.run_code_string(
                        _kybra_scope.clone(),
                        MAIN_PY,
                        "".to_owned(),
                    ).unwrap();
                });

                _KYBRA_INTERPRETER_OPTION = Some(_kybra_interpreter);
                _KYBRA_SCOPE_OPTION = Some(_kybra_scope);
            }
        }

        #[ic_cdk_macros::post_upgrade]
        fn _kybra_post_upgrade() {
            unsafe {
                let _kybra_interpreter = rustpython::vm::Interpreter::without_stdlib(Default::default());
                let _kybra_scope = _kybra_interpreter.enter(|vm| vm.new_scope_with_builtins());

                _kybra_interpreter.enter(|vm| {
                    vm.run_code_string(
                        _kybra_scope.clone(),
                        MAIN_PY,
                        "".to_owned(),
                    ).unwrap();
                });

                _KYBRA_INTERPRETER_OPTION = Some(_kybra_interpreter);
                _KYBRA_SCOPE_OPTION = Some(_kybra_scope);
            }
        }

        #[ic_cdk_macros::query]
        fn test(x: bool, y: bool) -> bool {
            unsafe {
                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                let result = _kybra_interpreter.enter(|vm| {
                    let hello_world_py_object_ref = _kybra_scope.globals.get_item("test", vm).unwrap();

                    let result_py_object_ref = vm.invoke(&hello_world_py_object_ref, (x.try_into_vm_value(vm).unwrap(), y.try_into_vm_value(vm).unwrap())).unwrap();

                    result_py_object_ref.try_from_vm_value(vm).unwrap()
                });

                result
            }
        }
    }
}
