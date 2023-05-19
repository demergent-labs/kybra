use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<(), &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<(), CdkActTryFromVmValueError> {
                Ok(())
            }
        }

        impl CdkActTryFromVmValue<bool, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<bool, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("TypeError: Could not convert PyObjectRef to bool".to_string())) // TODO consider using the try_into_value err
                }
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Empty, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Empty, CdkActTryFromVmValueError> {
                Err(CdkActTryFromVmValueError("TypeError: PyObjectRef cannot be converted into Empty".to_string()))
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Func, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Func, CdkActTryFromVmValueError> {
                let tuple_self: rustpython_vm::builtins::PyTupleRef = self.try_into_value(vm).unwrap_or_trap(vm);
                let principal = match tuple_self.get(0) {
                    Some(principal) => principal,
                    None => return Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Func. Missing Principal".to_string()))
                };
                let method = match tuple_self.get(1) {
                    Some(method) => method,
                    None => return Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Func. Missing method".to_string()))
                };

                Ok(ic_cdk::export::candid::Func {
                    principal: principal.clone().try_from_vm_value(vm)?,
                    method: method.clone().try_from_vm_value(vm)?
                })
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::Principal, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::Principal, CdkActTryFromVmValueError> {
                let to_str = self.get_attr("to_str", vm).unwrap_or_trap(vm);
                let result = to_str.call((), vm).unwrap_or_trap(vm);
                let result_string: String = result.try_into_value(vm).unwrap_or_trap(vm);
                match ic_cdk::export::Principal::from_text(result_string) {
                    Ok(principal) => Ok(principal),
                    Err(err) => Err(CdkActTryFromVmValueError(format!("TypeError: Could not convert value to Principal: {}", err.to_string()))),
                }
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Reserved, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Reserved, CdkActTryFromVmValueError> {
                Ok(ic_cdk::export::candid::Reserved)
            }
        }

        impl CdkActTryFromVmValue<ic_cdk_timers::TimerId, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk_timers::TimerId, CdkActTryFromVmValueError> {
                let vm_value_as_u64: u64 = self.try_into_value(vm).unwrap_or_trap(vm);
                Ok(ic_cdk_timers::TimerId::from(slotmap::KeyData::from_ffi(vm_value_as_u64)))
            }
        }

        impl CdkActTryFromVmValue<String, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<String, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("TypeError: Could not convert PyObjectRef to String".to_string())) // TODO consider using the try_into_value err
                }
            }
        }

        impl CdkActTryFromVmValue<Result<(), String>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Result<(), String>, CdkActTryFromVmValueError> {
                let err = self.get_item("Err", vm);
                if let Ok(error_message) = err {
                    return Ok(Err(error_message.try_from_vm_value(vm)?));
                }
                let ok = self.get_item("Ok", vm);
                if let Ok(value) = ok {
                    let result: Result<(), CdkActTryFromVmValueError> = value.try_from_vm_value(vm);
                    match result {
                        Ok(_) => Ok(Ok(())),
                        Err(err) => Err(CdkActTryFromVmValueError(format!("TypeError: Could not convert PyObjectRef to Result<(), String>. Ok value was not (). {:?}", err)))
                    }
                }
                else {
                    Err(CdkActTryFromVmValueError("TypeError: Could not convert PyObjectRef to Result<(), String>".to_string()))
                }
            }
        }
    }
}
