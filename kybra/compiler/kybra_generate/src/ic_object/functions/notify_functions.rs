use cdk_framework::{
    act::node::{
        candid::{service::Method, Service},
        Context,
    },
    traits::ToTypeAnnotation,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{keywords, tuple};

pub fn generate(services: &Vec<Service>) -> Vec<TokenStream> {
    services
        .iter()
        .map(|canister| {
            canister
                .methods
                .iter()
                .map(|method| {
                    let function_name_string = format!("notify_{}_{}", canister.name, method.name);
                    let real_function_name = format_ident!("{}", function_name_string);
                    let wrapper_fn_name = format_ident!("{}_wrapper", function_name_string);
                    let param_variable_definitions =
                        generate_param_variables(method, &canister.name);
                    let param_names = method
                        .params
                        .iter()
                        .map(|param| {
                            let name = format_ident!("{}", param.get_prefixed_name());
                            quote! {#name}
                        })
                        .collect();
                    let params = tuple::generate_tuple(&param_names);

                    quote! {
                        #[pymethod]
                        fn #wrapper_fn_name(
                            &self,
                            args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
                            vm: &rustpython_vm::VirtualMachine
                        ) -> rustpython_vm::PyResult {
                            let canister_id_principal: candid::Principal =
                                args_py_object_refs[0].clone().try_from_vm_value(vm)?;

                            #(#param_variable_definitions)*

                            let notify_result = #real_function_name(
                                canister_id_principal,
                                #params
                            );

                            notify_result
                                .try_into_vm_value(vm)
                                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
                        }
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<TokenStream>>>()
        .concat()
}

fn generate_param_variables(method: &Method, canister_name: &String) -> Vec<TokenStream> {
    method
        .params
        .iter()
        .enumerate()
        .map(|(index, param)| {
            let variable_name = format_ident!("{}", param.get_prefixed_name());
            let context = Context {
                keyword_list: keywords::get_python_keywords(),
                cdk_name: "kybra".to_string(),
            };
            let variable_type =
                param.to_type_annotation(&context, method.create_qualified_name(canister_name));
            let actual_index = index + 2;

            quote! {
                let #variable_name: (#variable_type) = args_py_object_refs[#actual_index]
                    .clone()
                    .try_from_vm_value(vm)?;
            }
        })
        .collect()
}
