use quote::quote;

pub fn generate_candid_decode() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn candid_decode(&self, candid_encoded_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let candid_encoded: Vec<u8> = candid_encoded_py_object_ref.try_from_vm_value(vm).unwrap();
            let candid_args: candid::IDLArgs = candid::IDLArgs::from_bytes(&candid_encoded).unwrap();
            let candid_string = candid_args.to_string();

            candid_string.try_into_vm_value(vm).unwrap()
        }
    }
}
