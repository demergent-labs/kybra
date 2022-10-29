use quote::quote;

pub fn generate_notify_raw() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn notify_raw(&self, canister_id_py_object_ref: PyObjectRef, method_py_object_ref: PyObjectRef, args_raw_py_object_ref: PyObjectRef, payment_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = canister_id_py_object_ref.try_from_vm_value(vm).unwrap();
            let method_string: String = method_py_object_ref.try_from_vm_value(vm).unwrap();
            let args_raw_vec: Vec<u8> = args_raw_py_object_ref.try_from_vm_value(vm).unwrap();
            let payment: u128 = payment_py_object_ref.try_from_vm_value(vm).unwrap();

            let notify_result = ic_cdk::api::call::notify_raw(
                canister_id_principal,
                &method_string,
                &args_raw_vec,
                payment
            );

            notify_result.try_into_vm_value(vm).unwrap()
        }
    }
}
