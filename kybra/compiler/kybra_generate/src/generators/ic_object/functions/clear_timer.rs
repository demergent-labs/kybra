use quote::quote;

pub fn generate_clear_timer() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_clear_timer(
            &self,
            timer_id_py_object_ref: PyObjectRef,
            vm: &VirtualMachine
        ) -> PyObjectRef {
            let timer_id: ic_cdk::timer::TimerId = timer_id_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::timer::clear_timer(timer_id).try_into_vm_value(vm).unwrap()
        }
    }
}
