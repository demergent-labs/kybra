use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        trait KybraTryIntoVec {}

        impl KybraTryIntoVec for () {}

        impl KybraTryIntoVec for bool {}

        impl KybraTryIntoVec for String {}

        impl KybraTryIntoVec for candid::Empty {}

        impl KybraTryIntoVec for candid::Reserved {}

        impl KybraTryIntoVec for candid::Func {}

        impl KybraTryIntoVec for candid::Principal {}

        impl KybraTryIntoVec for ic_cdk_timers::TimerId {}

        impl KybraTryIntoVec for ic_cdk::api::call::RejectionCode {}

        impl KybraTryIntoVec for f64 {}

        impl KybraTryIntoVec for f32 {}

        impl KybraTryIntoVec for _CdkFloat64 {}

        impl KybraTryIntoVec for _CdkFloat32 {}

        impl KybraTryIntoVec for candid::Int {}

        impl KybraTryIntoVec for i128 {}

        impl KybraTryIntoVec for i64 {}

        impl KybraTryIntoVec for i32 {}

        impl KybraTryIntoVec for i16 {}

        impl KybraTryIntoVec for i8 {}

        impl KybraTryIntoVec for candid::Nat {}

        impl KybraTryIntoVec for u128 {}

        impl KybraTryIntoVec for u64 {}

        impl KybraTryIntoVec for usize {}

        impl KybraTryIntoVec for u32 {}

        impl KybraTryIntoVec for u16 {}

        impl<T> KybraTryIntoVec for Option<T> {}

        impl<T> KybraTryIntoVec for Box<T> {}

        impl<T> KybraTryIntoVec for Vec<T> {}

        impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
            for Vec<T>
        where
            T: KybraTryIntoVec,
            T: for<'a> CdkActTryIntoVmValue<
                &'a rustpython::vm::VirtualMachine,
                rustpython::vm::PyObjectRef,
            >,
        {
            fn try_into_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                try_into_vm_value_generic_array(self, vm)
            }
        }

        impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
            for Vec<u8>
        {
            fn try_into_vm_value(
                self,
                vm: &rustpython::vm::VirtualMachine,
            ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
                Ok(vm.ctx.new_bytes(self).into())
            }
        }


        fn try_into_vm_value_generic_array<T>(
            generic_array: Vec<T>,
            vm: &rustpython::vm::VirtualMachine,
        ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError>
        where
            T: for<'a> CdkActTryIntoVmValue<
                &'a rustpython::vm::VirtualMachine,
                rustpython::vm::PyObjectRef,
            >,
        {
            let py_object_refs_result: Result<Vec<rustpython_vm::PyObjectRef>, CdkActTryIntoVmValueError> = generic_array
                .into_iter()
                .map(|item| item.try_into_vm_value(vm))
                .collect();

            Ok(vm.ctx.new_list(py_object_refs_result?).into())
        }
    }
}
