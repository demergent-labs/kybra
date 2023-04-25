use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn print(
            &self,
            param_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) {
            let param_string: String = param_py_object_ref.try_into_value(vm).unwrap();
            ic_cdk::println!("{:#?}", param_string);
        }
    }
}
