use quote::quote;

pub fn generate_canister_balance128() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn canister_balance128(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::canister_balance128().try_into_vm_value(vm).unwrap()
        }
    }
}
