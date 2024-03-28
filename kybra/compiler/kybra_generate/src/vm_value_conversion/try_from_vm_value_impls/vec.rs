use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<Vec<bool>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<bool>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<String>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<String>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<f64>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f64>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<f32>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f32>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<i128>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i128>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<i64>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i64>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<i32>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i32>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<i16>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i16>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<i8>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i8>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<u128>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u128>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<u64>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u64>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<u32>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u32>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<u16>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u16>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<u8>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u8>, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        trait KybraTryFromVec {}

        impl<T> KybraTryFromVec for Vec<T> {}

        impl<T> KybraTryFromVec for Box<T> {}

        impl KybraTryFromVec for () {}

        impl<T> KybraTryFromVec for Option<T> {}

        impl KybraTryFromVec for candid::Empty {}

        impl KybraTryFromVec for candid::Reserved {}

        impl KybraTryFromVec for candid::Func {}

        impl KybraTryFromVec for candid::Principal {}

        impl KybraTryFromVec for ic_cdk_timers::TimerId {}

        impl KybraTryFromVec for candid::Int {}

        impl KybraTryFromVec for candid::Nat {}

        impl KybraTryFromVec for _CdkFloat32 {}

        impl KybraTryFromVec for _CdkFloat64 {}

        impl<T> CdkActTryFromVmValue<Vec<T>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            T: KybraTryFromVec,
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, rustpython_vm::builtins::PyBaseExceptionRef, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, rustpython_vm::builtins::PyBaseExceptionRef> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        fn try_from_vm_value_generic_array<T>(py_object_ref: rustpython::vm::PyObjectRef, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, rustpython_vm::builtins::PyBaseExceptionRef>
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, rustpython_vm::builtins::PyBaseExceptionRef, &'a rustpython::vm::VirtualMachine>
        {
            let py_list: rustpython_vm::builtins::PyListRef = py_object_ref.try_into_value(vm)?;
            let vec = py_list.borrow_vec();

            vec.iter().map(|item| {
                match item.clone().try_from_vm_value(vm) {
                    Ok(item) => Ok(item),
                    Err(_) => Err(vm.new_type_error("Could not convert value to Vec".to_string()))
                }
            }).collect()
        }
    }
}
