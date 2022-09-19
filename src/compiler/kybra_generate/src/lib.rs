use quote::quote;

mod cdk_act;

pub fn kybra_generate(main_py: &str) -> proc_macro2::token_stream::TokenStream {
    // TODO keep it simple
    // TODO go get the functions and create the function bodies
    // TODO do everything with just strings
    // TODO try to follow the py_ast, cdk_act path that azle is following
    // TODO try not to do work that cdk_act will do for us

    // TODO the absolute first step is to get a python AST and walk it to get the functions
    // TODO then we determine if those functions are query or update functions
    // TODO then we create those functions' token streams

    quote! {
        use rustpython;

        static MAIN_PY: &'static str = #main_py;

        fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
            Ok(())
        }

        getrandom::register_custom_getrandom!(custom_getrandom);

        #[ic_cdk_macros::query]
        fn simple_query() -> String {
            let interpreter = rustpython::vm::Interpreter::without_stdlib(Default::default());

            let result = interpreter.enter(|vm| {
                let scope = vm.new_scope_with_builtins();

                vm.run_code_string(
                    scope.clone(),
                    MAIN_PY,
                    "".to_owned(),
                )
                .unwrap();

                let result_value = scope.globals.get_item("result", vm).unwrap();
                let result_string: String = result_value.try_into_value(vm).unwrap();

                result_string
            });

            result
        }
    }
}
