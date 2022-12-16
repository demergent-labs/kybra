use quote::quote;

pub fn generate_set_timer() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_set_timer(
            &self,
            delay_py_object_ref: PyObjectRef,
            func_py_object_ref: PyObjectRef,
            vm: &VirtualMachine
        ) -> PyObjectRef {
            let delay_as_u64: u64 = delay_py_object_ref.try_from_vm_value(vm).unwrap();
            let delay = core::time::Duration::new(delay_as_u64, 0);

            // TODO: Wrap the func_py_object_ref in a closure and actually pass
            // it to the set_timer API.
            // let timer_id = timers::set_timer(delay, func_py_object_ref, false)
            let timer_id = ic_cdk::timer::TimerId::default();

            ic_cdk::println!("Set_timer called");
            timer_id.try_into_vm_value(vm).unwrap()
        }
    }
}
