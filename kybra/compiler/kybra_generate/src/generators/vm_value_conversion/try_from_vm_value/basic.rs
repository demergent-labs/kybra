pub fn generate_basic_impls() -> proc_macro2::TokenStream {
    quote::quote! {
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
    }
}
