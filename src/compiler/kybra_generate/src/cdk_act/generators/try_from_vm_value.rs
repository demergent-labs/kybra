pub fn generate_try_from_vm_value() -> proc_macro2::TokenStream {
    quote::quote! {
        pub trait CdkActTryFromVmValue<T> {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<T, CdkActTryFromVmValueError>;
        }

        #[derive(Debug)]
        pub struct CdkActTryFromVmValueError(pub String);
    }
}
