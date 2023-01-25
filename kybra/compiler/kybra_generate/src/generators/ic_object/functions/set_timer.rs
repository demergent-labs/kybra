use quote::quote;

pub fn generate_set_timer() -> proc_macro2::TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_set_timer(
            &self,
            delay_py_object_ref: PyObjectRef,
            func_py_object_ref: PyObjectRef,
            vm: &VirtualMachine
        ) -> PyObjectRef {
            let delay_as_u64: u64 = delay_py_object_ref.try_from_vm_value(vm).unwrap();
            let delay = core::time::Duration::new(delay_as_u64, 0);

            let closure = move || {
                unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                    let vm = &_kybra_interpreter.vm;

                    let result_py_object_ref = vm.invoke(&func_py_object_ref, ());

                    match result_py_object_ref {
                        Ok(py_object_ref) => {
                            ic_cdk::spawn(async move {
                                _kybra_async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
                            });
                        },
                        Err(err) => {
                            let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                            panic!("{}", err_string);
                        }
                    };
                }
            };

            ic_cdk::timer::set_timer(delay, closure).try_into_vm_value(vm).unwrap()
        }
    }
}
