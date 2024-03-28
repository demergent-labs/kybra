use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<(), rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<(), rustpython_vm::builtins::PyBaseExceptionRef> {
                if self.is(&vm.ctx.none()) {
                    Ok(())
                } else {
                    let type_name = self.to_pyobject(vm).class().name().to_string();

                    Err(vm.new_type_error(format!(
                        "expected NoneType but received {type_name}"
                    )))
                }
            }
        }

        impl CdkActTryFromVmValue<bool, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<bool, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<candid::Empty, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<candid::Empty, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                Err(vm.new_type_error(
                    "value cannot be converted to Empty".to_string(),
                ))
            }
        }

        impl CdkActTryFromVmValue<candid::Func, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<candid::Func, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let tuple_self: rustpython_vm::builtins::PyTupleRef = self.try_into_value(vm)?;
                let principal = match tuple_self.get(0) {
                    Some(principal) => principal,
                    None => {
                        return Err(vm.new_type_error(
                            "could not convert value to Func, missing Principal"
                                .to_string(),
                        ))
                    }
                };
                let method = match tuple_self.get(1) {
                    Some(method) => method,
                    None => {
                        return Err(vm.new_type_error(
                            "could not convert value to Func, missing method"
                                .to_string(),
                        ))
                    }
                };

                Ok(candid::Func {
                    principal: principal.clone().try_from_vm_value(vm)?,
                    method: method.clone().try_from_vm_value(vm)?,
                })
            }
        }

        impl CdkActTryFromVmValue<candid::Principal, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<candid::Principal, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let to_str = self.get_attr("to_str", vm)?;
                let result = to_str.call((), vm)?;
                let result_string: String = result.try_into_value(vm)?;
                match candid::Principal::from_text(result_string) {
                    Ok(principal) => Ok(principal),
                    Err(err) => Err(vm.new_type_error(format!(
                        "could not convert value to Principal: {}",
                        err.to_string()
                    ))),
                }
            }
        }

        impl CdkActTryFromVmValue<candid::Reserved, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<candid::Reserved, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                Ok(candid::Reserved)
            }
        }

        impl CdkActTryFromVmValue<ic_cdk_timers::TimerId, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<ic_cdk_timers::TimerId, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let vm_value_as_u64: u64 = self.try_into_value(vm)?;
                Ok(ic_cdk_timers::TimerId::from(slotmap::KeyData::from_ffi(
                    vm_value_as_u64,
                )))
            }
        }

        impl CdkActTryFromVmValue<String, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<String, rustpython_vm::builtins::PyBaseExceptionRef> {
                self.try_into_value(vm)
            }
        }

        impl CdkActTryFromVmValue<Result<(), String>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<Result<(), String>, rustpython_vm::builtins::PyBaseExceptionRef>
            {
                let err = self.get_item("Err", vm);
                if let Ok(error_message) = err {
                    return Ok(Err(error_message.try_from_vm_value(vm)?));
                }

                let ok = self.get_item("Ok", vm);
                if let Ok(value) = ok {
                    let result: () = value.try_from_vm_value(vm)?;
                    return Ok(Ok(()));
                }

                let type_name = self.to_pyobject(vm).class().name().to_string();
                Err(vm.new_type_error(format!(
                    "expected Result but received {type_name}"
                )))
            }
        }
    }
}
