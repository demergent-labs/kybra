use cdk_framework::{nodes::act_external_canister::ActExternalCanister, ToTokenStream};
use quote::{format_ident, quote};

// TODO remember that cdk_framework needs a way to define the _azle or _kybra or whatever prefix

pub fn generate_async_result_handler(
    external_canisters: &Vec<ActExternalCanister>,
) -> proc_macro2::TokenStream {
    let call_match_arms = generate_call_match_arms(external_canisters);
    let call_with_payment_match_arms = generate_call_with_payment_match_arms(external_canisters);
    let call_with_payment128_match_arms =
        generate_call_with_payment128_match_arms(external_canisters);

    quote! {
        #[async_recursion::async_recursion(?Send)]
        async fn _kybra_async_result_handler(vm: &rustpython::vm::VirtualMachine, py_object_ref: &rustpython::vm::PyObjectRef, arg: PyObjectRef) -> rustpython::vm::PyObjectRef {
            if _kybra_is_generator(vm, &py_object_ref) == false {
                return py_object_ref.clone();
            }

            let send_result = vm.call_method(&py_object_ref, "send", (arg.clone(),));
            let py_iter_return = unwrap_rust_python_result(PyIterReturn::from_pyresult(send_result, vm), vm);

            match py_iter_return {
                PyIterReturn::Return(returned_py_object_ref) => {
                    if _kybra_is_generator(vm, &returned_py_object_ref) == true {
                        let recursed_py_object_ref = _kybra_async_result_handler(vm, &returned_py_object_ref, vm.ctx.none()).await;

                        return _kybra_async_result_handler(vm, py_object_ref, recursed_py_object_ref).await;
                    }

                    let name: String = returned_py_object_ref.get_attr("name", vm).unwrap().try_from_vm_value(vm).unwrap();
                    let args: Vec<PyObjectRef> = returned_py_object_ref.get_attr("args", vm).unwrap().try_into_value(vm).unwrap();

                    match &name[..] {
                        "call" => _kybra_async_result_handler_call(vm, py_object_ref, &args).await,
                        "call_with_payment" => _kybra_async_result_handler_call_with_payment(vm, py_object_ref, &args).await,
                        "call_with_payment128" => _kybra_async_result_handler_call_with_payment128(vm, py_object_ref, &args).await,
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

        // TODO some _azle prefixes need to be changed once the cdk_framework is updated
        async fn _kybra_async_result_handler_call(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();

            let cross_canister_call_function_name = format!("_azle_call_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_function_name[..] {
                #(#call_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        // TODO some _azle prefixes need to be changed once the cdk_framework is updated
        async fn _kybra_async_result_handler_call_with_payment(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();

            let cross_canister_call_with_payment_function_name = format!("_azle_call_with_payment_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_with_payment_function_name[..] {
                #(#call_with_payment_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        // TODO some _azle prefixes need to be changed once the cdk_framework is updated
        async fn _kybra_async_result_handler_call_with_payment128(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();

            let cross_canister_call_with_payment128_function_name = format!("_azle_call_with_payment128_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_with_payment128_function_name[..] {
                #(#call_with_payment128_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
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

            _kybra_async_result_handler(vm, py_object_ref, create_call_result_instance(vm, call_raw_result)).await
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

            _kybra_async_result_handler(vm, py_object_ref, create_call_result_instance(vm, call_raw_result)).await
        }

        fn create_call_result_instance<T>(vm: &rustpython::vm::VirtualMachine, call_result: CallResult<T>) -> PyObjectRef
            where T: for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
        {
            let canister_result_class = vm.run_block_expr(
                vm.new_scope_with_builtins(),
                r#"
from kybra import CanisterResult

CanisterResult
                "#
            ).unwrap();

            match call_result {
                Ok(ok) => {
                    let method_result = vm.invoke(&canister_result_class, (ok.try_into_vm_value(vm).unwrap(), vm.ctx.none()));

                    unwrap_rust_python_result(method_result, vm)

                    // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("ok", ok.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                },
                Err(err) => {
                    let err_string = format!("Rejection code {rejection_code}, {error_message}", rejection_code = (err.0 as i32).to_string(), error_message = err.1);

                    let method_result = vm.invoke(&canister_result_class, (vm.ctx.none(), err_string.try_into_vm_value(vm).unwrap()));

                    unwrap_rust_python_result(method_result, vm)

                    // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("err", err_string.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                }
            }
        }
    }
}

fn generate_call_match_arms(
    external_canisters: &Vec<ActExternalCanister>,
) -> Vec<proc_macro2::TokenStream> {
    external_canisters
    .iter()
    .map(|act_external_canister| {
        let canister_name = &act_external_canister.name;

        let arms: Vec<proc_macro2::TokenStream> = act_external_canister
            .methods
            .iter()
            .map(|act_external_canister_method| {
                let cross_canister_function_call_name = format!(
                    "_azle_call_{}_{}",
                    canister_name, act_external_canister_method.name
                );

                let cross_canister_function_call_name_ident = format_ident!("{}", cross_canister_function_call_name);

                let param_variable_definitions: Vec<proc_macro2::TokenStream> = act_external_canister_method.params.iter().enumerate().map(|(index, act_fn_param)| {
                    let variable_name = format_ident!("_kybra_{}", act_fn_param.name);
                    let variable_type = act_fn_param.data_type.to_token_stream(&vec![]);
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap();
                    }
                }).collect();

                let param_names: Vec<proc_macro2::TokenStream> = act_external_canister_method.params.iter().map(|act_fn_param| {
                    let param_name = format_ident!("_kybra_{}", act_fn_param.name);

                    quote!(#param_name)
                }).collect();

                quote! {
                    #cross_canister_function_call_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();

                        #(#param_variable_definitions)*

                        create_call_result_instance(vm, #cross_canister_function_call_name_ident(canister_id_principal, #(#param_names),*).await)
                    }
                }
            })
            .collect();

        quote! {
            #(#arms)*
        }
    })
    .collect()
}

fn generate_call_with_payment_match_arms(
    external_canisters: &Vec<ActExternalCanister>,
) -> Vec<proc_macro2::TokenStream> {
    external_canisters
    .iter()
    .map(|act_external_canister| {
        let canister_name = &act_external_canister.name;

        let arms: Vec<proc_macro2::TokenStream> = act_external_canister
            .methods
            .iter()
            .map(|act_external_canister_method| {
                let cross_canister_function_call_with_payment_name = format!(
                    "_azle_call_with_payment_{}_{}",
                    canister_name, act_external_canister_method.name
                );

                let cross_canister_function_call_with_payment_name_ident = format_ident!("{}", cross_canister_function_call_with_payment_name);

                let param_variable_definitions: Vec<proc_macro2::TokenStream> = act_external_canister_method.params.iter().enumerate().map(|(index, act_fn_param)| {
                    let variable_name = format_ident!("_kybra_{}", act_fn_param.name);
                    let variable_type = act_fn_param.data_type.to_token_stream(&vec![]);
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap();
                    }
                }).collect();

                let param_names: Vec<proc_macro2::TokenStream> = act_external_canister_method.params.iter().map(|act_fn_param| {
                    let param_name = format_ident!("_kybra_{}", act_fn_param.name);

                    quote!(#param_name)
                }).collect();

                let payment_comma = if act_external_canister_method.params.len() == 0 { quote! {} } else { quote! { , } };
                let payment_index = act_external_canister_method.params.len() + 2;
                let payment_variable_definition = quote!(let payment: u64 = args[#payment_index].clone().try_from_vm_value(vm).unwrap(););

                quote! {
                    #cross_canister_function_call_with_payment_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();

                        #(#param_variable_definitions)*
                        #payment_variable_definition

                        create_call_result_instance(vm, #cross_canister_function_call_with_payment_name_ident(canister_id_principal, #(#param_names),* #payment_comma payment).await)
                    }
                }
            })
            .collect();

        quote! {
            #(#arms)*
        }
    })
    .collect()
}

fn generate_call_with_payment128_match_arms(
    external_canisters: &Vec<ActExternalCanister>,
) -> Vec<proc_macro2::TokenStream> {
    external_canisters
    .iter()
    .map(|act_external_canister| {
        let canister_name = &act_external_canister.name;

        let arms: Vec<proc_macro2::TokenStream> = act_external_canister
            .methods
            .iter()
            .map(|act_external_canister_method| {
                let cross_canister_function_call_with_payment128_name = format!(
                    "_azle_call_with_payment128_{}_{}",
                    canister_name, act_external_canister_method.name
                );

                let cross_canister_function_call_with_payment128_name_ident = format_ident!("{}", cross_canister_function_call_with_payment128_name);

                let param_variable_definitions: Vec<proc_macro2::TokenStream> = act_external_canister_method.params.iter().enumerate().map(|(index, act_fn_param)| {
                    let variable_name = format_ident!("_kybra_{}", act_fn_param.name);
                    let variable_type = act_fn_param.data_type.to_token_stream(&vec![]);
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap();
                    }
                }).collect();

                let param_names: Vec<proc_macro2::TokenStream> = act_external_canister_method.params.iter().map(|act_fn_param| {
                    let param_name = format_ident!("_kybra_{}", act_fn_param.name);

                    quote!(#param_name)
                }).collect();

                let payment_comma = if act_external_canister_method.params.len() == 0 { quote! {} } else { quote! { , } };
                let payment_index = act_external_canister_method.params.len() + 2;
                let payment_variable_definition = quote!(let payment: u128 = args[#payment_index].clone().try_from_vm_value(vm).unwrap(););

                quote! {
                    #cross_canister_function_call_with_payment128_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();

                        #(#param_variable_definitions)*
                        #payment_variable_definition

                        create_call_result_instance(vm, #cross_canister_function_call_with_payment128_name_ident(canister_id_principal, #(#param_names),* #payment_comma payment).await)
                    }
                }
            })
            .collect();

        quote! {
            #(#arms)*
        }
    })
    .collect()
}
