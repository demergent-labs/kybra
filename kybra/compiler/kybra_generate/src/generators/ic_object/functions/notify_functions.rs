use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use cdk_framework::{
    nodes::{ActExternalCanister, ActExternalCanisterMethod},
    ToTokenStream,
};

pub fn generate_notify_functions(
    external_canisters: &Vec<ActExternalCanister>,
) -> Vec<TokenStream> {
    external_canisters.iter().map(|canister| {
        canister.methods.iter().map(|method| {
            let function_name_string = format!("_azle_notify_{}_{}", canister.name, method.name);
            let real_function_name = format_ident!("{}", function_name_string);
            let wrapper_fn_name = format_ident!("{}_wrapper", function_name_string);
            let param_variable_definitions = generate_param_variables(method);
            let param_names: Vec<TokenStream> = method.params.iter().map(|param| {
                let name = format_ident!("_kybra_{}", param.name);
                quote! { #name }
            }).collect();

            quote!{
                #[pymethod]
                fn #wrapper_fn_name(&self, args_py_object_refs: Vec<PyObjectRef>, vm: &VirtualMachine) -> PyObjectRef {
                    let canister_id_principal: ic_cdk::export::Principal = args_py_object_refs[0].clone().try_from_vm_value(vm).unwrap();

                    #(#param_variable_definitions)*

                    let notify_result = #real_function_name(
                        canister_id_principal,
                        #(#param_names),*
                    );

                    notify_result.try_into_vm_value(vm).unwrap()
                }
            }
        }).collect()
    }).collect::<Vec<Vec<TokenStream>>>().concat()
}

fn generate_param_variables(method: &ActExternalCanisterMethod) -> Vec<TokenStream> {
    method.params.iter().enumerate().map(|(index, act_fn_param)| {
        let variable_name = format_ident!("_kybra_{}", act_fn_param.name);
        let variable_type = act_fn_param.data_type.to_token_stream(&vec![]);
        let actual_index = index + 2;

        quote! {
            let #variable_name: #variable_type = args_py_object_refs[#actual_index].clone().try_from_vm_value(vm).unwrap();
        }
    }).collect()
}
