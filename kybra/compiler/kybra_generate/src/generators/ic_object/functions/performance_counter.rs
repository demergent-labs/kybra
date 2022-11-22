use quote::quote;

pub fn generate_performance_counter() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn performance_counter(&self, counter_type_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let counter_type: u32 = counter_type_py_object_ref.try_from_vm_value(vm).unwrap();

            ic_cdk::api::call::performance_counter(counter_type).try_into_vm_value(vm).unwrap()
        }
    }
}
