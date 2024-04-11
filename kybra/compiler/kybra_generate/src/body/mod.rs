use cdk_framework::act::node::{
    candid::Service,
    canister_method::{QueryMethod, UpdateMethod},
};
use proc_macro2::TokenStream;

use crate::{ic_object, stable_b_tree_map_nodes::rust, StableBTreeMapNode};

mod async_result_handler;
mod call_global_python_function;
mod guard_against_non_controllers;
mod unwrap_rust_python_result;
mod utils;

pub fn generate(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
    services: &Vec<Service>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> TokenStream {
    let async_result_handler = async_result_handler::generate(&services);
    let call_global_python_function = call_global_python_function::generate();
    let guard_against_non_controllers = guard_against_non_controllers::generate();
    let ic_object = ic_object::generate(
        update_methods,
        query_methods,
        services,
        stable_b_tree_map_nodes,
    );
    let stable_b_tree_map = rust::generate(stable_b_tree_map_nodes);
    let unwrap_rust_python_result = unwrap_rust_python_result::generate();
    let utils = utils::generate();

    quote::quote! {
        #async_result_handler
        #call_global_python_function
        #guard_against_non_controllers
        #ic_object
        #stable_b_tree_map
        #unwrap_rust_python_result
        #utils
    }
}
