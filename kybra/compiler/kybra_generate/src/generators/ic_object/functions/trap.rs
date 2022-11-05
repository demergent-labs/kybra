use quote::quote;

pub fn generate_trap() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn trap(&self, message_py_object_ref: PyObjectRef, vm: &VirtualMachine) {
            let message: String = message_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::trap(&message);
        }
    }
}
