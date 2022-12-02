use quote::quote;

pub fn generate_msg_cycles_available() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_msg_cycles_available(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::msg_cycles_available().try_into_vm_value(vm).unwrap()
        }
    }
}
