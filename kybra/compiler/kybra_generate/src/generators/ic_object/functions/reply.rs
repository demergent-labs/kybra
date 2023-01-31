use cdk_framework::{
    act::node::canister_method::{QueryMethod, UpdateMethod},
    ToTokenStream,
};
use quote::quote;

pub fn generate(
    canister_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
) -> proc_macro2::TokenStream {
    let match_arms = generate_match_arms(canister_methods, query_methods);
    quote! {
        #[pymethod]
        fn _kybra_reply(&self, first_called_function_name_py_object_ref: PyObjectRef, reply_value_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let first_called_function_name: String = first_called_function_name_py_object_ref.try_from_vm_value(vm).unwrap();

            match &first_called_function_name[..] {
                #(#match_arms)*
                _ => panic!("This cannot happen")
            }
        }
    }
}

fn generate_match_arms(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
) -> Vec<proc_macro2::TokenStream> {
    vec![
        update_methods
            .iter()
            .filter(|canister_method| canister_method.is_manual)
            .map(|canister_method| generate_update_match_arm(canister_method))
            .collect::<Vec<_>>(),
        query_methods
            .iter()
            .filter(|query_method| query_method.is_manual)
            .map(|query_method| generate_query_match_arm(query_method))
            .collect(),
    ]
    .concat()
}

fn generate_update_match_arm(update_method: &UpdateMethod) -> proc_macro2::TokenStream {
    let name = &update_method.name;
    let return_type = &update_method
        .return_type
        .to_token_stream(&crate::get_python_keywords());
    quote!(
        #name => {
            let reply_value: #return_type = reply_value_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::call::reply((reply_value,)).try_into_vm_value(vm).unwrap()
        }
    )
}

fn generate_query_match_arm(query_method: &QueryMethod) -> proc_macro2::TokenStream {
    let name = &query_method.name;
    let return_type = &query_method
        .return_type
        .to_token_stream(&crate::get_python_keywords());
    quote!(
        #name => {
            let reply_value: #return_type = reply_value_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::call::reply((reply_value,)).try_into_vm_value(vm).unwrap()
        }
    )
}
