use quote::quote;

pub fn generate_msg_cycles_available128() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn msg_cycles_available128(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::msg_cycles_available128().try_into_vm_value(vm).unwrap()
        }
    }
}
