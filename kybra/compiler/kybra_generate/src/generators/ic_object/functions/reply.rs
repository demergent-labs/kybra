use cdk_framework::{ActCanisterMethod, ToTokenStream};
use quote::quote;

pub fn generate_reply(canister_methods: &Vec<ActCanisterMethod>) -> proc_macro2::TokenStream {
    let match_arms = generate_match_arms(canister_methods);
    quote! {
        #[pymethod]
        fn reply(&self, first_called_function_name_py_object_ref: PyObjectRef, reply_value_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let first_called_function_name: String = first_called_function_name_py_object_ref.try_from_vm_value(vm).unwrap();

            match &first_called_function_name[..] {
                #(#match_arms)*
                _ => panic!("This cannot happen")
            }
        }
    }
}

fn generate_match_arms(canister_methods: &Vec<ActCanisterMethod>) -> Vec<proc_macro2::TokenStream> {
    canister_methods
        .iter()
        .filter(|canister_method| canister_method.is_manual())
        .map(|canister_method| generate_match_arm(canister_method))
        .collect()
}

fn generate_match_arm(canister_method: &ActCanisterMethod) -> proc_macro2::TokenStream {
    let name = &canister_method.get_name();
    let return_type = &canister_method.get_return_type().to_token_stream(&vec![]);
    quote!(
        #name => {
            let reply_value: #return_type = reply_value_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::call::reply((reply_value,)).try_into_vm_value(vm).unwrap()
        }
    )
}
