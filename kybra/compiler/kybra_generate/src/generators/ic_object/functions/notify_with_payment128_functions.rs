use cdk_framework::{
    act::node::{ExternalCanister, ExternalCanisterMethod},
    ToTokenStream,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generators::tuple;

pub fn generate(external_canisters: &Vec<ExternalCanister>) -> Vec<TokenStream> {
    external_canisters.iter().map(|canister| {
        canister.methods.iter().map(|method| {
            let function_name_string = format!("_kybra_notify_with_payment128_{}_{}", canister.name, method.name);
            let real_function_name = format_ident!("{}", function_name_string);
            let wrapper_fn_name = format_ident!("{}_wrapper", function_name_string);
            let param_variable_definitions = generate_param_variables(method);
            let param_names = method.params.iter().map(|param| {
                let name = format_ident!("{}", param.prefixed_name());
                quote!{ #name }
            }).collect();
            let params = tuple::generate_tuple(&param_names);

            quote!{
                #[pymethod]
                fn #wrapper_fn_name(&self, args_py_object_refs: Vec<PyObjectRef>, vm: &VirtualMachine) -> PyObjectRef {
                    let canister_id_principal: ic_cdk::export::Principal = args_py_object_refs[0].clone().try_from_vm_value(vm).unwrap();

                    #(#param_variable_definitions)*

                    let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1].clone().try_from_vm_value(vm).unwrap();

                    let notify_result = #real_function_name(
                        canister_id_principal,
                        #params,
                        cycles
                    );

                    notify_result.try_into_vm_value(vm).unwrap()
                }
            }
        }).collect()
    }).collect::<Vec<Vec<TokenStream>>>().concat()
}

fn generate_param_variables(method: &ExternalCanisterMethod) -> Vec<TokenStream> {
    method.params.iter().enumerate().map(|(index, act_fn_param)| {
        let variable_name = format_ident!("{}", act_fn_param.prefixed_name());
        let variable_type = act_fn_param.data_type.to_token_stream(&crate::get_python_keywords());
        let actual_index = index + 2;

        quote! {
            let #variable_name: #variable_type = args_py_object_refs[#actual_index].clone().try_from_vm_value(vm).unwrap();
        }
    }).collect()
}
