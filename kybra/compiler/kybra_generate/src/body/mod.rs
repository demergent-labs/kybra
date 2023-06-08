use cdk_framework::act::node::{
    candid::Service,
    canister_method::{QueryMethod, UpdateMethod},
};
use proc_macro2::TokenStream;

use crate::{
    async_result_handler, ic_object, stable_b_tree_map_nodes::rust, unwrap_rust_python_result,
    utils, StableBTreeMapNode,
};

pub fn generate(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
    services: &Vec<Service>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> TokenStream {
    let ic_object = ic_object::generate(
        update_methods,
        query_methods,
        services,
        stable_b_tree_map_nodes,
    );
    let unwrap_rust_python_result = unwrap_rust_python_result::generate();
    let async_result_handler = async_result_handler::generate(&services);
    let stable_b_tree_map = rust::generate(stable_b_tree_map_nodes);
    let utils = utils::generate();

    quote::quote! {
        #ic_object
        #unwrap_rust_python_result
        #async_result_handler
        #stable_b_tree_map
        #utils
    }
}
