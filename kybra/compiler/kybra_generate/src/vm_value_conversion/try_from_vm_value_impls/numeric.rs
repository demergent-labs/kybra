use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<f64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<f64, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<_CdkFloat64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<_CdkFloat64, rustpython_vm::builtins::PyBaseExceptionRef> {
                Ok(_CdkFloat64(self.try_into_value(vm)?))
            }
        }

        impl CdkActTryFromVmValue<f32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<f32, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<_CdkFloat32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<_CdkFloat32, rustpython_vm::builtins::PyBaseExceptionRef> {
                Ok(_CdkFloat32(self.try_into_value(vm)?))
            }
        }

        impl CdkActTryFromVmValue<candid::Int, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<candid::Int, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let int_result: Result<rustpython_vm::builtins::PyIntRef, _> =
                    self.try_into_value(vm);

                match int_result {
                    Ok(int) => Ok(candid::Int(int.as_bigint().clone())),
                    Err(_) => Err(vm.new_type_error("PyObjectRef is not a PyIntRef".to_string())),
                }
            }
        }

        impl CdkActTryFromVmValue<i128, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i128, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<i64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i64, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<i32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i32, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<i16, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i16, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<i8, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<i8, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<candid::Nat, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<candid::Nat, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let int: rustpython_vm::builtins::PyIntRef = self.try_into_value(vm)?;

                match candid::Nat::from_str(&int.as_bigint().to_string()) {
                    // TODO probably not the best conversion
                    Ok(nat) => Ok(nat),
                    Err(_) => {
                        Err(vm.new_type_error("Could not convert value to nat".to_string()))
                    }
                }
            }
        }

        impl CdkActTryFromVmValue<u128, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u128, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<u64, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u64, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<usize, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<usize, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<u32, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u32, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<u16, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u16, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<u8, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<u8, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }
    }
}
