use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl<T> CdkActTryFromVmValue<(T,), &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<(T,), CdkActTryFromVmValueError> {
                match self.try_from_vm_value(vm) {
                    Ok(value) => Ok((value,)),
                    Err(_) => Err(CdkActTryFromVmValueError("TypeError: Could not convert value to tuple".to_string()))
                }
            }
        }

        impl<T> CdkActTryFromVmValue<Box<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Box<T>, CdkActTryFromVmValueError> {
                match self.try_from_vm_value(vm) {
                    Ok(value) => Ok(Box::new(value)),
                    Err(err) => Err(err)
                }
            }
        }

        impl<T> CdkActTryFromVmValue<Option<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Option<T>, CdkActTryFromVmValueError> {
                if self.is(&vm.ctx.none()) {
                    Ok(None)
                }
                else {
                    match self.try_from_vm_value(vm) {
                        Ok(value) => Ok(Some(value)),
                        Err(err) => Err(err)
                    }
                }
            }
        }

        impl<T> CdkActTryFromVmValue<ic_cdk::api::call::ManualReply<T>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<ic_cdk::api::call::ManualReply<T>, CdkActTryFromVmValueError> {
                Ok(ic_cdk::api::call::ManualReply::empty())
            }
        }
    }
}
