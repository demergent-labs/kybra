use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn clear_timer(
            &self,
            timer_id_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let timer_id: ic_cdk_timers::TimerId = timer_id_py_object_ref.try_from_vm_value(vm)?;

            ic_cdk_timers::clear_timer(timer_id)
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
