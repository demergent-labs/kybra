use quote::quote;

pub fn generate_canister_balance() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn canister_balance(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::canister_balance().try_into_vm_value(vm).unwrap()
        }
    }
}
