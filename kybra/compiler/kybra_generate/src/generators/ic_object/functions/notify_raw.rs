use quote::quote;

pub fn generate_ic_object_function_notify_raw() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn notify_raw(&self, canister_id: PyObjectRef, method: PyObjectRef, args_raw: PyObjectRef, payment: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = canister_id.try_from_vm_value(vm).unwrap();
            let method_string: String = method.try_from_vm_value(vm).unwrap();
            let args_raw_vec: Vec<u8> = args_raw.try_from_vm_value(vm).unwrap();
            let payment: u128 = payment.try_from_vm_value(vm).unwrap();

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
