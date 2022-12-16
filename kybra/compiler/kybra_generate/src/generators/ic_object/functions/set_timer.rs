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

            let func__name__attr = _kybra_unwrap_rust_python_result(func_py_object_ref.get_attr("__name__", vm), vm);
            let func_name: String = func__name__attr.try_from_vm_value(vm).unwrap();

            let closure = move || {
                unsafe {
                    let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                    let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();
                    let vm = &_kybra_interpreter.vm;

                    let one_time_timer_callback_py_object_ref =
                        _kybra_unwrap_rust_python_result(_kybra_scope.globals.get_item(&func_name, vm), vm);
                    let invoke_result = vm.invoke(
                        &one_time_timer_callback_py_object_ref,
                        (),
                    );
                    match invoke_result {
                        Ok(py_object_ref) => (),
                        Err(err) => {
                            let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();
                            panic!("{}", err_string);
                        }
                    }
                }
            };

            ic_cdk::timer::set_timer(delay, closure).try_into_vm_value(vm).unwrap()
        }
    }
}
