use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
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
                Err(CdkActTryIntoVmValueError("type \"empty\" cannot be represented in python".to_string()))
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::export::candid::Func {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(vm.ctx.new_tuple(vec![self.principal.try_into_vm_value(vm).unwrap_or_trap(), self.method.try_into_vm_value(vm).unwrap_or_trap()]).into())
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk::export::Principal {
            // TODO: In the future CdkActTryIntoVmValue needs to return rustpython_vm::object::PyResult
            // When it does all these map_err calls will be unnecessary and should be replaced with
            // question mark syntax.
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                let principal_class = vm.run_block_expr(
                    vm.new_scope_with_builtins(),
                    r#"
from kybra import Principal

Principal
                    "#
                ).map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?;

                let from_str = principal_class.get_attr("from_str", vm).map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?;
                let principal_instance = from_str.call((self.to_text(),), vm).map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?;

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

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_cdk_timers::TimerId {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(self.data().as_ffi().to_pyobject(vm))
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

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for ic_stable_structures::btreemap::InsertError {
            fn try_into_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                match self {
                    ic_stable_structures::btreemap::InsertError::KeyTooLarge {given, max} => {
                        let dict = vm.ctx.new_dict();

                        let key_too_large_dict = vm.ctx.new_dict();
                        key_too_large_dict.set_item("given", given.try_into_vm_value(vm).unwrap_or_trap(), vm);
                        key_too_large_dict.set_item("max", max.try_into_vm_value(vm).unwrap_or_trap(), vm);

                        dict.set_item("KeyTooLarge", key_too_large_dict.into(), vm);

                        Ok(dict.into())
                    },
                    ic_stable_structures::btreemap::InsertError::ValueTooLarge {given, max} => {
                        let dict = vm.ctx.new_dict();

                        let value_too_large_dict = vm.ctx.new_dict();
                        value_too_large_dict.set_item("given", given.try_into_vm_value(vm).unwrap_or_trap(), vm);
                        value_too_large_dict.set_item("max", max.try_into_vm_value(vm).unwrap_or_trap(), vm);

                        dict.set_item("ValueTooLarge", value_too_large_dict.into(), vm);

                        Ok(dict.into())
                    }
                }
            }
        }
    }
}
