pub fn generate_vec_impls() -> proc_macro2::TokenStream {
    quote::quote! {
        impl CdkActTryFromVmValue<Vec<()>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<()>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<bool>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<bool>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<String>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<String>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<ic_cdk::export::candid::Empty>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<ic_cdk::export::candid::Empty>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<ic_cdk::export::candid::Reserved>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<ic_cdk::export::candid::Reserved>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<ic_cdk::export::candid::Func>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<ic_cdk::export::candid::Func>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<ic_cdk::export::Principal>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<ic_cdk::export::Principal>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<f64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f64>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<f32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<f32>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<ic_cdk::export::candid::Int>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<ic_cdk::export::candid::Int>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<i128>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i128>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<i64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i64>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<i32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i32>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<i16>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i16>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<i8>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<i8>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<ic_cdk::export::candid::Nat>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<ic_cdk::export::candid::Nat>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryFromVmValue<Vec<u128>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u128>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<u64>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u64>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<u32>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u32>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<u16>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u16>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        impl CdkActTryFromVmValue<Vec<u8>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<u8>, CdkActTryFromVmValueError> {
                Ok(self.try_into_value(vm).unwrap())
            }
        }

        // TODO consider creating a macro that can derive Vec of Vec multiple levels deep for any type
        impl CdkActTryFromVmValue<Vec<Vec<u8>>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<Vec<u8>>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array(self, vm)
            }
        }

        impl<T> CdkActTryFromVmValue<Vec<Box<T>>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<Box<T>>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array::<Box<T>>(self, vm)
            }
        }

        impl<T> CdkActTryFromVmValue<Vec<Option<T>>, &rustpython::vm::VirtualMachine> for rustpython::vm::PyObjectRef
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            fn try_from_vm_value(self, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<Option<T>>, CdkActTryFromVmValueError> {
                try_from_vm_value_generic_array::<Option<T>>(self, vm)
            }
        }

        fn try_from_vm_value_generic_array<T>(py_object_ref: rustpython::vm::PyObjectRef, vm: &rustpython::vm::VirtualMachine) -> Result<Vec<T>, CdkActTryFromVmValueError>
        where
            rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<T, &'a rustpython::vm::VirtualMachine>
        {
            let py_list: PyListRef = py_object_ref.try_into_value(vm).unwrap();
            let vec = py_list.borrow_vec();

            Ok(vec.iter().map(|item| {
                item.clone().try_from_vm_value(vm).unwrap()
            }).collect())
        }
    }
}
