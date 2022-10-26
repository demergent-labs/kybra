pub fn generate_numeric_impls() -> proc_macro2::TokenStream {
    quote::quote! {
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

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Int, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Int, CdkActTryFromVmValueError> {
                let int_result: Result<PyIntRef, _> = self.try_into_value(vm);

                match int_result {
                    Ok(int) => Ok(ic_cdk::export::candid::Int(int.as_bigint().clone())),
                    Err(_) => Err(CdkActTryFromVmValueError("PyObjectRef is not a PyIntRef".to_string()))
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

        impl CdkActTryFromVmValue<ic_cdk::export::candid::Nat, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::export::candid::Nat, CdkActTryFromVmValueError> {
                let int_result: Result<PyIntRef, _> = self.try_into_value(vm);

                match int_result {
                    Ok(int) => Ok(ic_cdk::export::candid::Nat::from_str(&int.as_bigint().to_string()).unwrap()), // TODO probably not the best conversion
                    Err(_) => Err(CdkActTryFromVmValueError("PyObjectRef is not a PyIntRef".to_string()))
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

        impl CdkActTryFromVmValue<usize, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<usize, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(CdkActTryFromVmValueError("Could not convert PyObjectRef to usize".to_string()))
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
