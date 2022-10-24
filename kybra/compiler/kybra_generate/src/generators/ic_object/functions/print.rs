use quote::quote;

pub fn generate_ic_object_function_print() -> proc_macro2::TokenStream {
    quote! {
        #[pyclass]
        impl Ic {
            #[pymethod]
            fn print(&self, param_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
                let param_string: String = param_py_object_ref.try_into_value(vm).unwrap();
                ic_cdk::println!("{:#?}", param_string);
            }
        }
    }
}
