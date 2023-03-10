use quote::quote;

pub fn generate() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_data_certificate(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::data_certificate().try_into_vm_value(vm).unwrap()
        }
    }
}
