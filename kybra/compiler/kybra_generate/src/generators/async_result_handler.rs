pub fn generate_async_result_handler() -> proc_macro2::TokenStream {
    quote::quote! {
        #[async_recursion::async_recursion(?Send)]
        async fn _kybra_async_result_handler(vm: &rustpython::vm::VirtualMachine, py_object_ref: &rustpython::vm::PyObjectRef, arg: PyObjectRef) -> rustpython::vm::PyObjectRef {
            if _kybra_is_generator(vm, &py_object_ref) == false {
                return py_object_ref.clone();
            }

            let send_result = vm.call_method(py_object_ref.as_object(), "send", (arg.clone(),));
            let py_iter_return = PyIterReturn::from_pyresult(send_result, vm).unwrap();

            match py_iter_return {
                PyIterReturn::Return(returned_py_object_ref) => {
                    if _kybra_is_generator(vm, &returned_py_object_ref) == true {
                        let recursed_py_object_ref = _kybra_async_result_handler(vm, &returned_py_object_ref, vm.ctx.none()).await;

                        return _kybra_async_result_handler(vm, py_object_ref, recursed_py_object_ref).await;
                    }

                    let name: String = returned_py_object_ref.get_item("name", vm).unwrap().try_from_vm_value(vm).unwrap();
                    let args: Vec<PyObjectRef> = returned_py_object_ref.get_item("args", vm).unwrap().try_into_value(vm).unwrap();

                    match &name[..] {
                        "call_raw" => _kybra_async_result_handler_call_raw(vm, py_object_ref, &args).await,
                        "call_raw128" => _kybra_async_result_handler_call_raw128(vm, py_object_ref, &args).await,
                        _ => panic!("async operation not supported")
                    }
                },
                PyIterReturn::StopIteration(returned_py_object_ref) => returned_py_object_ref.unwrap() // TODO when would this return an option?
            }
        }

        // TODO do this more officially, check if py_object_ref instanceof generator type
        fn _kybra_is_generator(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef) -> bool {
            if let Ok(_) = py_object_ref.get_attr("send", vm) {
                true
            }
            else {
                false
            }
        }

        async fn _kybra_async_result_handler_call_raw(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let method_string: String = args[1].clone().try_from_vm_value(vm).unwrap();
            let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm).unwrap();
            let payment: u64 = args[3].clone().try_from_vm_value(vm).unwrap();

            let call_raw_result = ic_cdk::api::call::call_raw(
                canister_id_principal,
                &method_string,
                &args_raw_vec,
                payment
            ).await;

            let call_raw_result_tuple = match call_raw_result {
                Ok(ok) => {
                    vm.ctx.new_tuple(vec![ok.try_into_vm_value(vm).unwrap(), vm.ctx.none()])

                    // TODO use dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("ok", ok.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                },
                Err(err) => {
                    let err_string = format!("Rejection code {rejection_code}, {error_message}", rejection_code = (err.0 as i32).to_string(), error_message = err.1);

                    vm.ctx.new_tuple(vec![vm.ctx.none(), err_string.try_into_vm_value(vm).unwrap()])

                    // TODO use dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("err", err_string.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                }
            };

            _kybra_async_result_handler(vm, py_object_ref, call_raw_result_tuple.into()).await
        }

        async fn _kybra_async_result_handler_call_raw128(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let method_string: String = args[1].clone().try_from_vm_value(vm).unwrap();
            let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm).unwrap();
            let payment: u128 = args[3].clone().try_from_vm_value(vm).unwrap();

            let call_raw_result = ic_cdk::api::call::call_raw128(
                canister_id_principal,
                &method_string,
                &args_raw_vec,
                payment
            ).await;

            let call_raw_result_tuple = match call_raw_result {
                Ok(ok) => {
                    vm.ctx.new_tuple(vec![ok.try_into_vm_value(vm).unwrap(), vm.ctx.none()])

                    // TODO use dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("ok", ok.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                },
                Err(err) => {
                    let err_string = format!("Rejection code {rejection_code}, {error_message}", rejection_code = (err.0 as i32).to_string(), error_message = err.1);

                    vm.ctx.new_tuple(vec![vm.ctx.none(), err_string.try_into_vm_value(vm).unwrap()])

                    // TODO use dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("err", err_string.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                }
            };

            _kybra_async_result_handler(vm, py_object_ref, call_raw_result_tuple.into()).await
        }
    }
}
