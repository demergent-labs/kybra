use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use cdk_framework::traits::ToIdent;

pub fn to_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
            for #service_name
        {
            fn try_into_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                unsafe {
                    let scope = match SCOPE_OPTION.clone() {
                        Some(scope) => scope,
                        // TODO: Note: Because we return a CdkActTryIntoVmValueError instead of a
                        // PyResult, this will likely surface be displayed to the end user as:
                        // "TypeError: SystemError: missing python scope". The double categorization
                        // is weird. So, we should fix this once we return PyResults.
                        None => {
                            return Err(CdkActTryIntoVmValueError(
                                "SystemError: missing python scope".to_string(),
                            ))
                        }
                    };
                    Ok(vm
                        .run_block_expr(
                            scope,
                            format!(
                                "from kybra import Principal; {}(Principal.from_str('{}'))",
                                stringify!(#service_name),
                                self.0.principal.to_string()
                            )
                            .as_str(),
                        )
                        .map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?)
                }
            }
        }
    }
}

pub fn list_to_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
            for Vec<#service_name>
        {
            fn try_into_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }
    }
}

pub fn from_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<#service_name, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<#service_name, rustpython_vm::builtins::PyBaseExceptionRef> {
                let canister_id = self.get_attr("canister_id", vm)?;
                let to_str = canister_id.get_attr("to_str", vm)?;
                let result = to_str.call((), vm)?;
                let result_string: String = result.try_into_value(vm)?;
                match candid::Principal::from_text(result_string) {
                    Ok(principal) => Ok(#service_name::new(principal)),
                    Err(err) => Err(vm.new_type_error(format!(
                        "TypeError: Could not convert value to Principal: {}",
                        err.to_string()
                    ))),
                }
            }
        }
    }
}

pub fn list_from_vm_value(name: String) -> TokenStream {
    let service_name = name.to_ident().to_token_stream();
    quote! {
        impl CdkActTryFromVmValue<Vec<#service_name>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<Vec<#service_name>, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                try_from_vm_value_generic_array(self, vm)
            }
        }
    }
}
