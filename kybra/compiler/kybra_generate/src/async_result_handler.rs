use cdk_framework::act::{
    node::{candid::Service, Context},
    ToTypeAnnotation,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{keywords, tuple};

pub fn generate(services: &Vec<Service>) -> TokenStream {
    let call_match_arms = generate_call_match_arms(services);
    let call_with_payment_match_arms = generate_call_with_payment_match_arms(services);
    let call_with_payment128_match_arms = generate_call_with_payment128_match_arms(services);

    quote! {
        #[async_recursion::async_recursion(?Send)]
        async fn _kybra_async_result_handler(vm: &rustpython::vm::VirtualMachine, py_object_ref: &rustpython::vm::PyObjectRef, arg: PyObjectRef) -> rustpython::vm::PyObjectRef {
            if _kybra_is_generator(vm, &py_object_ref) == false {
                return py_object_ref.clone();
            }

            let send_result = vm.call_method(&py_object_ref, "send", (arg.clone(),));
            let py_iter_return = _kybra_unwrap_rust_python_result(PyIterReturn::from_pyresult(send_result, vm), vm);

            match py_iter_return {
                PyIterReturn::Return(returned_py_object_ref) => {
                    if _kybra_is_generator(vm, &returned_py_object_ref) == true {
                        let recursed_py_object_ref = _kybra_async_result_handler(vm, &returned_py_object_ref, vm.ctx.none()).await;

                        return _kybra_async_result_handler(vm, py_object_ref, recursed_py_object_ref).await;
                    }

                    let name: String = _kybra_unwrap_rust_python_result(returned_py_object_ref.get_attr("name", vm), vm).try_from_vm_value(vm).unwrap();
                    let args: Vec<PyObjectRef> = _kybra_unwrap_rust_python_result(_kybra_unwrap_rust_python_result(returned_py_object_ref.get_attr("args", vm), vm).try_into_value(vm), vm);

                    match &name[..] {
                        "call" => _kybra_async_result_handler_call(vm, py_object_ref, &args).await,
                        "call_with_payment" => _kybra_async_result_handler_call_with_payment(vm, py_object_ref, &args).await,
                        "call_with_payment128" => _kybra_async_result_handler_call_with_payment128(vm, py_object_ref, &args).await,
                        "call_raw" => _kybra_async_result_handler_call_raw(vm, py_object_ref, &args).await,
                        "call_raw128" => _kybra_async_result_handler_call_raw128(vm, py_object_ref, &args).await,
                        _ => panic!("async operation not supported")
                    }
                },
                PyIterReturn::StopIteration(returned_py_object_ref_option) => match returned_py_object_ref_option {
                    Some(returned_py_object_ref) => returned_py_object_ref,
                    None => vm.ctx.none()
                }
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

        async fn _kybra_async_result_handler_call(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();

            let cross_canister_call_function_name = format!("_kybra_call_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_function_name[..] {
                #(#call_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        async fn _kybra_async_result_handler_call_with_payment(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();

            let cross_canister_call_with_payment_function_name = format!("_kybra_call_with_payment_{}", qualname.replace(".", "_"));

            let call_result_instance = match &cross_canister_call_with_payment_function_name[..] {
                #(#call_with_payment_match_arms),*
                _ => panic!("cross canister function does not exist")
            };

            _kybra_async_result_handler(vm, py_object_ref, call_result_instance).await
        }

        async fn _kybra_async_result_handler_call_with_payment128(vm: &rustpython::vm::VirtualMachine, py_object_ref: &PyObjectRef, args: &Vec<PyObjectRef>) -> PyObjectRef {
            let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();
            let qualname: String = args[1].clone().try_from_vm_value(vm).unwrap();

            let cross_canister_call_with_payment128_function_name = format!("_kybra_call_with_payment128_{}", qualname.replace(".", "_"));

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

            _kybra_async_result_handler(vm, py_object_ref, _kybra_create_call_result_instance(vm, call_raw_result)).await
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

            _kybra_async_result_handler(vm, py_object_ref, _kybra_create_call_result_instance(vm, call_raw_result)).await
        }

        fn _kybra_create_call_result_instance<T>(vm: &rustpython::vm::VirtualMachine, call_result: CallResult<T>) -> PyObjectRef
            where T: for<'a> CdkActTryIntoVmValue<&'a rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
        {
            let call_result_class = _kybra_unwrap_rust_python_result(vm.run_block_expr(
                vm.new_scope_with_builtins(),
                r#"
from kybra import CallResult

CallResult
                "#
            ), vm);

            match call_result {
                Ok(ok) => {
                    let method_result = vm.invoke(&call_result_class, (ok.try_into_vm_value(vm).unwrap(), vm.ctx.none()));

                    _kybra_unwrap_rust_python_result(method_result, vm)

                    // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("Ok", ok.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                },
                Err(err) => {
                    let err_string = format!("Rejection code {rejection_code}, {error_message}", rejection_code = (err.0 as i32).to_string(), error_message = err.1);

                    let method_result = vm.invoke(&call_result_class, (vm.ctx.none(), err_string.try_into_vm_value(vm).unwrap()));

                    _kybra_unwrap_rust_python_result(method_result, vm)

                    // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                    // let dict = vm.ctx.new_dict();

                    // dict.set_item("Err", err_string.try_into_vm_value(vm).unwrap(), vm);

                    // dict
                }
            }
        }
    }
}

fn generate_call_match_arms(services: &Vec<Service>) -> Vec<TokenStream> {
    services
    .iter()
    .map(|service| {
        let canister_name = &service.name;

        let arms: Vec<TokenStream> = service
            .methods
            .iter()
            .map(|method| {
                let cross_canister_function_call_name = format!(
                    "_kybra_call_{}_{}",
                    canister_name, method.name
                );

                let cross_canister_function_call_name_ident = format_ident!("{}", cross_canister_function_call_name);

                let context = Context {
                    keyword_list: keywords::get_python_keywords(),
                    cdk_name: "kybra".to_string(),
                };

                let param_variable_definitions: Vec<TokenStream> = method.params.iter().enumerate().map(|(index, param)| {
                    let variable_name = format_ident!("{}", param.get_prefixed_name());
                    let variable_type = param.to_type_annotation(&context, method.create_qualified_name(&service.name));
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap();
                    }
                }).collect();

                let param_names = method.params.iter().map(|param| {
                    let name = format_ident!("{}", param.get_prefixed_name());
                    quote! {#name}
                }).collect();
                let params = tuple::generate_tuple(&param_names);

                quote! {
                    #cross_canister_function_call_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();

                        #(#param_variable_definitions)*

                        _kybra_create_call_result_instance(vm, #cross_canister_function_call_name_ident(canister_id_principal, #params).await)
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

fn generate_call_with_payment_match_arms(services: &Vec<Service>) -> Vec<TokenStream> {
    services
    .iter()
    .map(|service| {
        let canister_name = &service.name;

        let arms: Vec<TokenStream> = service
            .methods
            .iter()
            .map(|method| {
                let cross_canister_function_call_with_payment_name = format!(
                    "_kybra_call_with_payment_{}_{}",
                    canister_name, method.name
                );

                let cross_canister_function_call_with_payment_name_ident = format_ident!("{}", cross_canister_function_call_with_payment_name);

                let context = Context {
                    keyword_list: keywords::get_python_keywords(),
                    cdk_name: "kybra".to_string(),
                };

                let param_variable_definitions: Vec<TokenStream> = method.params.iter().enumerate().map(|(index, param)| {
                    let variable_name = format_ident!("{}", param.get_prefixed_name());
                    let variable_type = param.to_type_annotation(&context, method.create_qualified_name(&service.name));
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap();
                    }
                }).collect();

                let param_names: Vec<TokenStream> = method.params.iter().map(|param| {
                    let name = format_ident!("{}", param.get_prefixed_name());
                    quote! {#name}
                }).collect();
                let params = tuple::generate_tuple(&param_names);

                let payment_index = method.params.len() + 2;
                let payment_variable_definition = quote!(let payment: u64 = args[#payment_index].clone().try_from_vm_value(vm).unwrap(););

                quote! {
                    #cross_canister_function_call_with_payment_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();

                        #(#param_variable_definitions)*
                        #payment_variable_definition

                        _kybra_create_call_result_instance(vm, #cross_canister_function_call_with_payment_name_ident(canister_id_principal, #params, payment).await)
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

fn generate_call_with_payment128_match_arms(services: &Vec<Service>) -> Vec<TokenStream> {
    services
    .iter()
    .map(|service| {
        let canister_name = &service.name;

        let arms: Vec<TokenStream> = service
            .methods
            .iter()
            .map(|method| {
                let cross_canister_function_call_with_payment128_name = format!(
                    "_kybra_call_with_payment128_{}_{}",
                    canister_name, method.name
                );

                let cross_canister_function_call_with_payment128_name_ident = format_ident!("{}", cross_canister_function_call_with_payment128_name);

                let context = Context {
                    keyword_list: keywords::get_python_keywords(),
                    cdk_name: "kybra".to_string(),
                };

                let param_variable_definitions: Vec<TokenStream> = method.params.iter().enumerate().map(|(index, param)| {
                    let variable_name = format_ident!("{}", param.get_prefixed_name());
                    let variable_type = param.to_type_annotation(&context, method.create_qualified_name(&service.name));
                    let actual_index = index + 2;

                    quote! {
                        let #variable_name: #variable_type = args[#actual_index].clone().try_from_vm_value(vm).unwrap();
                    }
                }).collect();

                let param_names: Vec<TokenStream> = method.params.iter().map(|param| {
                    let name = format_ident!("{}", param.get_prefixed_name());
                    quote! {#name}
                }).collect();
                let params = tuple::generate_tuple(&param_names);

                let payment_index = method.params.len() + 2;
                let payment_variable_definition = quote!(let payment: u128 = args[#payment_index].clone().try_from_vm_value(vm).unwrap(););

                quote! {
                    #cross_canister_function_call_with_payment128_name => {
                        let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm).unwrap();

                        #(#param_variable_definitions)*
                        #payment_variable_definition

                        _kybra_create_call_result_instance(vm, #cross_canister_function_call_with_payment128_name_ident(canister_id_principal, #params, payment).await)
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
