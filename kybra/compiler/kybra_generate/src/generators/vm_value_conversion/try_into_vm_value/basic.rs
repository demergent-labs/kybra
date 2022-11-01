pub fn generate_basic_impls() -> proc_macro2::TokenStream {
    quote::quote! {
        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for () {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(vm.ctx.none())
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for bool {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(self.to_pyobject(vm))
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::export::candid::Empty {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                panic!("Empty cannot be converted into PyObjectRef");
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::export::candid::Func {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(vm.ctx.new_tuple(vec![self.principal.try_into_vm_value(vm).unwrap(), self.method.try_into_vm_value(vm).unwrap()]).into())
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::export::Principal {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                let principal_class = vm.run_block_expr(
                    vm.new_scope_with_builtins(),
                    r#"
from kybra import Principal

Principal
                    "#
                ).unwrap();

                let from_str = principal_class.get_attr("from_str", vm).unwrap();
                let principal_instance = vm.invoke(&from_str, (self.to_text(),)).unwrap();

                Ok(principal_instance)
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::api::call::RejectionCode {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                match self {
                    ic_cdk::api::call::RejectionCode::NoError => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("NoError", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::call::RejectionCode::SysFatal => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("SysFatal", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::call::RejectionCode::SysTransient => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("SysTransient", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::call::RejectionCode::DestinationInvalid => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("DestinationInvalid", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::call::RejectionCode::CanisterReject => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("CanisterReject", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::call::RejectionCode::CanisterError => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("CanisterError", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::call::RejectionCode::Unknown => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("Unknown", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                }
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::export::candid::Reserved {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(vm.ctx.none())
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::api::stable::StableMemoryError {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                match self {
                    ic_cdk::api::stable::StableMemoryError::OutOfMemory => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("OutOfMemory", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                    ic_cdk::api::stable::StableMemoryError::OutOfBounds => {
                        let dict = vm.ctx.new_dict();

                        dict.set_item("OutOfBounds", vm.ctx.none(), vm);

                        Ok(dict.into())
                    }
                }
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for String {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(self.to_pyobject(vm))
            }
        }
    }
}
