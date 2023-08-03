use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn msg_cycles_accept128(
            &self,
            max_amount_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let max_amount: u128 = max_amount_py_object_ref.try_from_vm_value(vm)?;

            ic_cdk::api::call::msg_cycles_accept128(max_amount)
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
