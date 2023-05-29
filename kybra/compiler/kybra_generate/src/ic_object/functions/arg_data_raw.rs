use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn arg_data_raw(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
            ic_cdk::api::call::arg_data_raw()
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
