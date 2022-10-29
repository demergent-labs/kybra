use quote::quote;

pub fn generate_ic_object_function_candid_encode() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn candid_encode(&self, param_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let candid_string: String = param_py_object_ref.try_from_vm_value(vm).unwrap();
            let candid_args: candid::IDLArgs = candid_string.parse().unwrap();
            let candid_encoded: Vec<u8> = candid_args.to_bytes().unwrap();

            candid_encoded.try_into_vm_value(vm).unwrap()
        }
    }
}
