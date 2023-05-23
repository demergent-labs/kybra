use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn method_name(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
            ic_cdk::api::call::method_name()
                .try_into_vm_value(vm)
                .map_err(|try_from_err| vm.new_type_error(try_from_err.0))
        }
    }
}
