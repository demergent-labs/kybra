use quote::quote;

pub fn generate_method_name() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn method_name(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::method_name().try_into_vm_value(vm).unwrap()
        }
    }
}
