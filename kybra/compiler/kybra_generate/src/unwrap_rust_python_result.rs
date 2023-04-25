use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        pub fn _kybra_unwrap_rust_python_result<T>(
            rust_python_result: Result<T, rustpython_vm::PyRef<rustpython_vm::builtins::PyBaseException>>,
            vm: &rustpython::vm::VirtualMachine
        ) -> T {
            match rust_python_result {
                Ok(ok) => ok,
                Err(err) => {
                    let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                    panic!("{}", err_string);
                },
            }
        }
    }
}
