use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn candid_encode(
            &self,
            candid_string_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let candid_string: String = candid_string_py_object_ref.try_from_vm_value(vm).unwrap();
            let candid_args: candid::IDLArgs = candid_string.parse().unwrap();
            let candid_encoded: Vec<u8> = candid_args.to_bytes().unwrap();

            candid_encoded.try_into_vm_value(vm).unwrap()
        }
    }
}
