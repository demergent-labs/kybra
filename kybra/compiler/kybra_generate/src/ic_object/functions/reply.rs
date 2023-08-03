use cdk_framework::{
    act::node::{
        canister_method::{QueryMethod, UpdateMethod},
        Context,
    },
    traits::ToTypeAnnotation,
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::keywords;

pub fn generate(
    canister_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
) -> TokenStream {
    let match_arms = generate_match_arms(canister_methods, query_methods);
    quote! {
        #[pymethod]
        fn reply(
            &self,
            first_called_function_name_py_object_ref: rustpython_vm::PyObjectRef,
            reply_value_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyResult {
            let first_called_function_name: String =
                first_called_function_name_py_object_ref.try_from_vm_value(vm)?;

            match &first_called_function_name[..] {
                #(#match_arms)*
                // TODO: Consider creating a custom error
                _ => Err(vm.new_system_error(format!(
                    "attempted to reply from \"{}\", but it does not appear to be a canister method",
                    first_called_function_name
                )))
            }
        }
    }
}

fn generate_match_arms(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
) -> Vec<TokenStream> {
    vec![
        update_methods
            .iter()
            .filter(|canister_method| canister_method.is_manual)
            .map(|update_method| generate_update_match_arm(update_method))
            .collect::<Vec<_>>(),
        query_methods
            .iter()
            .filter(|query_method| query_method.is_manual)
            .map(|query_method| generate_query_match_arm(query_method))
            .collect(),
    ]
    .concat()
}

fn generate_update_match_arm(update_method: &UpdateMethod) -> TokenStream {
    let name = &update_method.name;
    let context = Context {
        keyword_list: keywords::get_python_keywords(),
        cdk_name: "kybra".to_string(),
    };
    let return_type = update_method
        .return_type
        .to_type_annotation(&context, update_method.name.clone());
    quote!(
        #name => {
            let reply_value: (#return_type) = reply_value_py_object_ref.try_from_vm_value(vm)?;

            ic_cdk::api::call::reply((reply_value,))
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    )
}

fn generate_query_match_arm(query_method: &QueryMethod) -> TokenStream {
    let name = &query_method.name;
    let context = Context {
        keyword_list: keywords::get_python_keywords(),
        cdk_name: "kybra".to_string(),
    };
    let return_type = query_method
        .return_type
        .to_type_annotation(&context, query_method.name.clone());
    quote!(
        #name => {
            let reply_value: (#return_type) = reply_value_py_object_ref.try_from_vm_value(vm)?;
            ic_cdk::api::call::reply((reply_value,))
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
        }
    )
}
