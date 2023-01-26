use quote::quote;

pub fn generate() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_canister_balance128(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::canister_balance128().try_into_vm_value(vm).unwrap()
        }
    }
}
