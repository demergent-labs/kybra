use proc_macro2::TokenStream;
use quote::quote;

use crate::{stable_b_tree_map_nodes::rust, StableBTreeMapNode};

pub fn generate(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> TokenStream {
    let match_arms = generate_match_arms(stable_b_tree_map_nodes);

    quote! {
        #[pymethod]
        fn stable_b_tree_map_contains_key(
            &self,
            memory_id_py_object_ref: rustpython_vm::PyObjectRef,
            key_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();

            match memory_id {
                #(#match_arms)*
                _ => panic!("memory_id {} does not have an associated StableBTreeMap", memory_id)
            }
        }
    }
}

fn generate_match_arms(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> Vec<TokenStream> {
    stable_b_tree_map_nodes
        .iter()
        .map(|stable_b_tree_map_node| {
            let memory_id = stable_b_tree_map_node.memory_id;
            let map_name_ident = rust::ref_cell_ident::generate(stable_b_tree_map_node.memory_id);

            let (key_wrapper_type_name, _) = rust::wrapper_type::generate(&stable_b_tree_map_node.key_type, memory_id, "Key");

            quote! {
                #memory_id => {
                    #map_name_ident.with(|p| p.borrow().contains_key(&#key_wrapper_type_name(key_py_object_ref.try_from_vm_value(vm).unwrap()))).try_into_vm_value(vm).unwrap()
                }
            }
        })
        .collect()
}
