use cdk_framework::act::node::{
    canister_method::{QueryMethod, UpdateMethod},
    ExternalCanister,
};
use proc_macro2::TokenStream;

use crate::{
    async_result_handler, ic_object, stable_b_tree_map_nodes::stable_b_tree_map,
    unwrap_rust_python_result, StableBTreeMapNode,
};

pub fn generate(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
    external_canisters: &Vec<ExternalCanister>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> TokenStream {
    let ic_object = ic_object::generate(
        update_methods,
        query_methods,
        external_canisters,
        stable_b_tree_map_nodes,
    );
    let unwrap_rust_python_result = unwrap_rust_python_result::generate();
    let async_result_handler = async_result_handler::generate(&external_canisters);
    let stable_b_tree_map = stable_b_tree_map::generate(stable_b_tree_map_nodes);

    quote::quote! {
        #ic_object
        #unwrap_rust_python_result
        #async_result_handler
        #stable_b_tree_map
    }
}
