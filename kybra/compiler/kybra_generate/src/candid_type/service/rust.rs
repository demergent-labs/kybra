use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use cdk_framework::traits::ToIdent;

pub fn to_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #service_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                unsafe {
                    let scope = SCOPE_OPTION.clone().unwrap();
                    Ok(unwrap_rust_python_result(vm.run_block_expr(
                        scope, format!(
r#"
from kybra import Principal
{}(Principal.from_str('{}'))
"#, stringify!(#service_name), self.0.principal.to_string()).as_str()), vm))
                }
            }
        }
    }
}

pub fn list_to_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Vec<#service_name> {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }
    }
}

pub fn from_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<#service_name, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#service_name, CdkActTryFromVmValueError> {
                let canister_id = unwrap_rust_python_result(self.get_attr("canister_id", vm), vm);
                let to_str = unwrap_rust_python_result(canister_id.get_attr("to_str", vm), vm);
                let result = unwrap_rust_python_result(vm.invoke(&to_str, ()), vm);
                let result_string: String = unwrap_rust_python_result(result.try_into_value(vm), vm);

                Ok(#service_name::new(ic_cdk::export::Principal::from_str(&result_string).unwrap()))
            }
        }
    }
}

pub fn list_from_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<Vec<#service_name>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#service_name>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}
