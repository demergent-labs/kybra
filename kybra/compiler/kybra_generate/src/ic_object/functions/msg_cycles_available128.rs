use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn msg_cycles_available128(
            &self,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyResult {
            ic_cdk::api::call::msg_cycles_available128()
            .try_into_vm_value(vm)
            .map_err(|try_from_err| vm.new_type_error(try_from_err.0))
        }
    }
}
