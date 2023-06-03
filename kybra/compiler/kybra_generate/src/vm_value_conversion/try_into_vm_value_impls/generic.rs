use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for (T,)
        where
            T : for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>,
        {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                self.0.try_into_vm_value(vm)
            }
        }

        impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Box<T>
        where
            T : for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>,
        {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                (*self).try_into_vm_value(vm)
            }
        }

        impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Option<T>
        where
            T: for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
        {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                match self {
                    Some(value) => value.try_into_vm_value(vm),
                    None => Ok(().to_pyobject(vm))
                }
            }
        }

        impl<T, K> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for Result<T, K>
        where
            T: for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>,
            K: for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
        {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                match self {
                    Ok(ok) => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("Ok", ok.try_into_vm_value(vm)?, vm);

                        Ok(dict.into())
                    },
                    Err(err) => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("Err", err.try_into_vm_value(vm)?, vm);

                        Ok(dict.into())
                    }
                }
            }
        }
    }
}
