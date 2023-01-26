use cdk_framework::nodes::ActExternalCanister;

use crate::{
    generators::{
        async_result_handler::generate_async_result_handler, kybra_serde::generate_kybra_serde,
        rng_seed, stable_b_tree_map::generate_stable_b_tree_map,
    },
    py_ast::kybra_types::StableBTreeMapNode,
};

pub fn generate(
    external_canisters: &Vec<ActExternalCanister>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> proc_macro2::TokenStream {
    let async_result_handler = generate_async_result_handler(&external_canisters);
    let kybra_serde = generate_kybra_serde();
    let stable_b_tree_map = generate_stable_b_tree_map(stable_b_tree_map_nodes);
    let rng_seed = rng_seed::generate();

    quote::quote! {
        pub fn _kybra_unwrap_rust_python_result<T>(
            rust_python_result: Result<T, PyRef<PyBaseException>>,
            vm: &rustpython::vm::VirtualMachine
        ) -> T {
            match rust_python_result {
                Ok(ok) => ok,
                Err(err) => {
                    let err_string: String = err.to_pyobject(vm).repr(vm).unwrap().to_string();

                    panic!("{}", err_string);
                },
            }
        }

        #async_result_handler

        #kybra_serde

        #stable_b_tree_map

        #rng_seed
    }
}
