use quote::quote;

use crate::{stable_b_tree_map_nodes::stable_b_tree_map, StableBTreeMapNode};

pub fn generate(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> proc_macro2::TokenStream {
    let match_arms = generate_match_arms(stable_b_tree_map_nodes);

    quote! {
        #[pymethod]
        fn _kybra_stable_b_tree_map_values(&self, memory_id_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> Vec<PyObjectRef> {
            let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();

            match memory_id {
                #(#match_arms)*
                _ => panic!("memory_id {} does not have an associated StableBTreeMap", memory_id)
            }
        }
    }
}

fn generate_match_arms(
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> Vec<proc_macro2::TokenStream> {
    stable_b_tree_map_nodes
        .iter()
        .map(|stable_b_tree_map_node| {
            let memory_id = stable_b_tree_map_node.memory_id;
            let map_name_ident = stable_b_tree_map::ref_cell_ident::generate(stable_b_tree_map_node.memory_id);

            quote! {
                #memory_id => {
                    #map_name_ident.with(|p| p.borrow().iter().map(|(_, value_wrapper_type)| value_wrapper_type.0.try_into_vm_value(vm).unwrap()).collect())
                }
            }
        })
        .collect()
}
