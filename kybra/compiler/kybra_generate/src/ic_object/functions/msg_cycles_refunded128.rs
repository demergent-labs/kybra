use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_msg_cycles_refunded128(&self, vm: &VirtualMachine) -> PyObjectRef {
            ic_cdk::api::call::msg_cycles_refunded128().try_into_vm_value(vm).unwrap()
        }
    }
}
