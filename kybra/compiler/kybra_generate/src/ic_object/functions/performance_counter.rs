use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_performance_counter(&self, counter_type_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let counter_type: u32 = counter_type_py_object_ref.try_from_vm_value(vm).unwrap();

            ic_cdk::api::call::performance_counter(counter_type).try_into_vm_value(vm).unwrap()
        }
    }
}
