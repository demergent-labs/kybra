use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use cdk_framework::nodes::data_type_nodes::ToIdent;

pub fn generate_func_to_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for #type_alias_name {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                self.0.try_into_vm_value(vm)
            }
        }
    }
}

pub fn generate_func_list_to_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Vec<#type_alias_name> {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }
    }
}

pub fn generate_func_from_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<#type_alias_name, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<#type_alias_name, CdkActTryFromVmValueError> {
                let candid_func: candid::Func = self.try_from_vm_value(vm).unwrap();
                Ok(candid_func.into())
            }
        }
    }
}

pub fn generate_func_list_from_vm_value(name: &String) -> TokenStream {
    let type_alias_name = name.to_identifier().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<Vec<#type_alias_name>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<#type_alias_name>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}
