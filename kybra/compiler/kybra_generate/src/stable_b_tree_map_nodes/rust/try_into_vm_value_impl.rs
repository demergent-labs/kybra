use proc_macro2::TokenStream;
use syn::Ident;

pub fn generate(wrapper_type_name: &Ident) -> TokenStream {
    quote::quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #wrapper_type_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(self.0.try_into_vm_value(vm)?)
            }
        }
    }
}
