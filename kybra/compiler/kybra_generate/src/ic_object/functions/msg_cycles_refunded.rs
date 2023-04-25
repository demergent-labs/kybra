use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_msg_cycles_refunded(
            &self,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            ic_cdk::api::call::msg_cycles_refunded().try_into_vm_value(vm).unwrap()
        }
    }
}
