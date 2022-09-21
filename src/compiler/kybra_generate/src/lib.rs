use cdk_act::generators::{
    try_from_vm_value::generate_try_from_vm_value, try_into_vm_value::generate_try_into_vm_value,
};
use generators::{
    act::generate_act, try_from_vm_value_impl::generate_try_from_vm_value_impl,
    try_into_vm_value_impl::generate_try_into_vm_value_impl,
};
use quote::quote;

mod cdk_act;
mod generators;

pub fn kybra_generate(main_py: &str) -> proc_macro2::token_stream::TokenStream {
    let act = generate_act(main_py);
    let act_token_stream = act.to_token_stream();

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

        #act_token_stream
    }
}
