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
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to bool".to_string())) // TODO consider using the try_into_value err
                }
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Empty, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Empty, CdkActTryFromVmValueError> {
                panic!("PyObjectRef cannot be converted into Empty");
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Func, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Func, CdkActTryFromVmValueError> {
                let tuple_self: PyTupleRef = _kybra_unwrap_rust_python_result(self.try_into_value(vm), vm);
                let principal = tuple_self.get(0).unwrap();
                let method = tuple_self.get(1).unwrap();

                Ok(ic_cdk::export::candid::Func {
                    principal: principal.clone().try_from_vm_value(vm).unwrap(),
                    method: method.clone().try_from_vm_value(vm).unwrap()
                })
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::Principal, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::Principal, CdkActTryFromVmValueError> {
                let to_str = _kybra_unwrap_rust_python_result(self.get_attr("to_str", vm), vm);
                let result = _kybra_unwrap_rust_python_result(vm.invoke(&to_str, ()), vm);
                let result_string: String = _kybra_unwrap_rust_python_result(result.try_into_value(vm), vm);
                Ok(ic_cdk::export::Principal::from_text(result_string).unwrap())
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Reserved, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Reserved, CdkActTryFromVmValueError> {
                Ok(ic_cdk::export::candid::Reserved)
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::timer::TimerId, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::timer::TimerId, CdkActTryFromVmValueError> {
                let vm_value_as_u64: u64 = _kybra_unwrap_rust_python_result(self.try_into_value(vm), vm);
                Ok(ic_cdk::timer::TimerId::from(slotmap::KeyData::from_ffi(vm_value_as_u64)))
            }
        }

        impl CdkActTryFromVmValue<String, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<String, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to String".to_string())) // TODO consider using the try_into_value err
                }
            }
        }

        impl CdkActTryFromVmValue<Result<(), String>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Result<(), String>, CdkActTryFromVmValueError> {
                let err = self.get_item("Err", vm);
                if let Ok(error_message) = err {
                    return Ok(Err(error_message.try_from_vm_value(vm).unwrap()));
                }
                let ok = self.get_item("Ok", vm);
                if let Ok(value) = ok {
                    let result: Result<(), CdkActTryFromVmValueError> = value.try_from_vm_value(vm);
                    match result {
                        Ok(_) => Ok(Ok(())),
                        Err(err) => Err(CdkActTryFromVmValueError(format!("Could not convert PyObjectRef to Result<(), String>. Ok value was not (). {:?}", err)))
                    }
                }
                else {
                    Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to Result<(), String>".to_string()))
                }
            }
        }
    }
}
