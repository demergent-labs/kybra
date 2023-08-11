use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl<T> CdkActTryFromVmValue<(T,), rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef:
                for<'a> CdkActTryFromVmValue<T, rustpython_vm::builtins::PyBaseExceptionRef, &'a rustpython::vm::VirtualMachine>,
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<(T,), rustpython_vm::builtins::PyBaseExceptionRef> {
                match self.try_from_vm_value(vm) {
                    Ok(value) => Ok((value,)),
                    Err(_) => {
                        Err(vm.new_type_error("Could not convert value to tuple".to_string()))
                    }
                }
            }
        }

        impl<T> CdkActTryFromVmValue<Box<T>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef:
                for<'a> CdkActTryFromVmValue<T, rustpython_vm::builtins::PyBaseExceptionRef, &'a rustpython::vm::VirtualMachine>,
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<Box<T>, rustpython_vm::builtins::PyBaseExceptionRef> {
                Ok(Box::new(self.try_from_vm_value(vm)?))
            }
        }

        impl<T> CdkActTryFromVmValue<Option<T>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef:
                for<'a> CdkActTryFromVmValue<T, rustpython_vm::builtins::PyBaseExceptionRef, &'a rustpython::vm::VirtualMachine>,
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<Option<T>, rustpython_vm::builtins::PyBaseExceptionRef> {
                if self.is(&vm.ctx.none()) {
                    Ok(None)
                } else {
                    Ok(Some(self.try_from_vm_value(vm)?))
                }
            }
        }

        impl<T>
            CdkActTryFromVmValue<ic_cdk::api::call::ManualReply<T>, rustpython_vm::builtins::PyBaseExceptionRef, &rustpython::vm::VirtualMachine>
            for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef:
                for<'a> CdkActTryFromVmValue<T, rustpython_vm::builtins::PyBaseExceptionRef, &'a rustpython::vm::VirtualMachine>,
        {
            fn try_from_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<
                ic_cdk::api::call::ManualReply<T>,
                rustpython_vm::builtins::PyBaseExceptionRef,
            > {
                Ok(ic_cdk::api::call::ManualReply::empty())
            }
        }
    }
}
