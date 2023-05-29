use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn notify_raw(
            &self,
            canister_id_py_object_ref: rustpython_vm::PyObjectRef,
            method_py_object_ref: rustpython_vm::PyObjectRef,
            args_raw_py_object_ref: rustpython_vm::PyObjectRef,
            payment_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let canister_id_principal: ic_cdk::export::Principal = canister_id_py_object_ref
                .try_from_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;

            let method_string: String = method_py_object_ref
                .try_from_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;

            let args_raw_vec: Vec<u8> = args_raw_py_object_ref
                .try_from_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;

            let payment: u128 = payment_py_object_ref
                .try_from_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;

            let notify_result = ic_cdk::api::call::notify_raw(
                canister_id_principal,
                &method_string,
                &args_raw_vec,
                payment,
            );

            notify_result
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
