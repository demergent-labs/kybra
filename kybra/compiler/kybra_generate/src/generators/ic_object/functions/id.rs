use quote::quote;

pub fn generate_id() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn id(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::id().try_into_vm_value(_context).unwrap()
        }
    }
}
