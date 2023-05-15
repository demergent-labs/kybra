use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        pub fn unwrap_rust_python_result<T>(
            rust_python_result: Result<T, rustpython_vm::PyRef<rustpython_vm::builtins::PyBaseException>>,
            vm: &rustpython::vm::VirtualMachine
        ) -> T {
            match rust_python_result {
                Ok(ok) => ok,
                Err(err) => {
                    let type_name = err.clone().to_pyobject(vm).class().name().to_string();
                    let err_message = match &err.to_pyobject(vm).str(vm) {
                        Ok(string) => string.to_string(),
                        Err(_) => ic_cdk::trap(format!("Attribute Error: '{}' object has no attribute '__str__'", type_name).as_str()),
                    };
                    ic_cdk::trap(format!("{type_name}: {err_message}").as_str())
                },
            }
        }
    }
}
