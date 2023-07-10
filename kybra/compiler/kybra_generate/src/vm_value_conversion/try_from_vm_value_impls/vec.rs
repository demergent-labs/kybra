use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<Vec<bool>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<bool>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<bool>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<String>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<String>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<String>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<f64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f64>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<f64>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<f32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f32>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<f32>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<i128>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i128>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<i128>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<i64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i64>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<i64>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<i32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i32>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<i32>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<i16>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i16>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<i16>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<i8>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i8>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<i8>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<u128>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u128>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<u128>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<u64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u64>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<u64>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<u32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u32>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<u32>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<u16>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u16>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<u16>".to_string()))
                }
            }
        }

        impl CdkActTryFromVmValue<Vec<u8>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u8>, CdkActTryFromVmValueError> {
                match self.try_into_value(vm) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec<u8>".to_string()))
                }
            }
        }

        trait KybraTryFromVec {}

        impl<T> KybraTryFromVec for Vec<T> {}

        impl<T> KybraTryFromVec for Box<T> {}

        impl KybraTryFromVec for () {}

        impl<T> KybraTryFromVec for Option<T> {}

        impl KybraTryFromVec for ic_cdk::export::candid::Empty {}

        impl KybraTryFromVec for ic_cdk::export::candid::Reserved {}

        impl KybraTryFromVec for ic_cdk::export::candid::Func {}

        impl KybraTryFromVec for ic_cdk::export::Principal {}

        impl KybraTryFromVec for ic_cdk_timers::TimerId {}

        impl KybraTryFromVec for ic_cdk::export::candid::Int {}

        impl KybraTryFromVec for ic_cdk::export::candid::Nat {}

        impl KybraTryFromVec for _CdkFloat32 {}

        impl KybraTryFromVec for _CdkFloat64 {}

        impl<T> CdkActTryFromVmValue<Vec<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            T: KybraTryFromVec,
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        fn try_from_vm_value_generic_array<T>(py_object_ref: rustpython::vm::PyObjectRef, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, CdkActTryFromVmValueError>
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            let py_list: rustpython_vm::builtins::PyListRef = py_object_ref.try_into_value(vm)
                .map_err(|err| err.to_cdk_act_try_from_vm_value_error(vm))?;
            let vec = py_list.borrow_vec();

            vec.iter().map(|item| {
                match item.clone().try_from_vm_value(vm) {
                    Ok(item) => Ok(item),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to Vec".to_string()))
                }
            }).collect()
        }
    }
}
