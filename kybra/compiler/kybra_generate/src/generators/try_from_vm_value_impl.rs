pub fn generate_try_from_vm_value_impl() -> proc_macro2::TokenStream {
    quote::quote! {
        // Basic types

        impl CdkActTryFromVmValue<(), &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<(), CdkActTryFromVmValueError> {
                // match self.try_into_value(vm) {
                //     Ok(value) => Ok(value),
                //     Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to ()".to_string())) // TODO consider using the try_into_value err
                // }
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

        impl CdkActTryFromVmValue<String, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<String, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to String".to_string())) // TODO consider using the try_into_value err
                }
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Empty, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Empty, CdkActTryFromVmValueError> {
                panic!("PyObjectRef cannot be converted into Empty");
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Reserved, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Reserved, CdkActTryFromVmValueError> {
                Ok(ic_cdk::export::candid::Reserved)
            }
        }

        impl CdkActTryFromVmValue<ic_cdk::export::Principal, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::Principal, CdkActTryFromVmValueError> {
                let to_str = self.get_attr("to_str", vm).unwrap();
                let result = vm.invoke(&to_str, ()).unwrap();
                let result_string: String = result.try_into_value(vm).unwrap();
                Ok(ic_cdk::export::Principal::from_text(result_string).unwrap())
            }
        }

        // Number types

        impl CdkActTryFromVmValue<f64, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<f64, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to f64".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<f32, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<f32, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to f32".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<i128, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<i128, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to i128".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<i64, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<i64, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to i64".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<i32, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<i32, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to i32".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<i16, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<i16, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to i16".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<i8, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<i8, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to i8".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<u128, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<u128, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to u128".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<u64, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<u64, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to u64".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<u32, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<u32, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to u32".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<u16, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<u16, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to u16".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<u8, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<u8, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to u8".to_string()))
                }
            }
        }
    }
}
