use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn data_certificate(
            &self,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyResult {
            ic_cdk::api::data_certificate()
                .try_into_vm_value(vm)
                .map_err(|try_from_err| vm.new_type_error(try_from_err.0))

        }
    }
}
