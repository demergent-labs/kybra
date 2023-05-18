use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn set_timer_interval(
            &self,
            interval_py_object_ref: rustpython_vm::PyObjectRef,
            func_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let interval_as_u64: u64 = interval_py_object_ref.try_from_vm_value(vm).unwrap_or_trap();
            let interval = core::time::Duration::new(interval_as_u64, 0);

            let closure = move || {
                unsafe {
                    let interpreter = INTERPRETER_OPTION
                        .as_mut()
                        .unwrap_or_trap("SystemError: missing python interpreter");
                    let scope = SCOPE_OPTION
                        .as_mut()
                        .unwrap_or_trap("SystemError: missing python scope");

                    let vm = &interpreter.vm;

                    let result_py_object_ref = vm.invoke(&func_py_object_ref, ());

                    match result_py_object_ref {
                        Ok(py_object_ref) => {
                            ic_cdk::spawn(async move {
                                async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
                            });
                        },
                        Err(err) => {
                            let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                            panic!("{}", err_string);
                        }
                    };
                }
            };

            ic_cdk_timers::set_timer_interval(interval, closure).try_into_vm_value(vm).unwrap_or_trap()
        }
    }
}
