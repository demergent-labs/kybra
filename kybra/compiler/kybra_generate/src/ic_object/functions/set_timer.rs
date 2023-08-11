use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn set_timer(
            &self,
            delay_py_object_ref: rustpython_vm::PyObjectRef,
            func_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let delay_as_u64: u64 = delay_py_object_ref.try_from_vm_value(vm)?;

            let delay = core::time::Duration::new(delay_as_u64, 0);

            let closure = move || {
                let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
                    .unwrap_or_trap("SystemError: missing python interpreter");
                let scope = unsafe { SCOPE_OPTION.as_mut() }
                    .unwrap_or_trap("SystemError: missing python scope");

                let vm = &interpreter.vm;

                let py_object_ref = func_py_object_ref.call((), vm).unwrap_or_trap(vm);

                ic_cdk::spawn(async move {
                    async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
                });
            };

            ic_cdk_timers::set_timer(delay, closure)
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    }
}
