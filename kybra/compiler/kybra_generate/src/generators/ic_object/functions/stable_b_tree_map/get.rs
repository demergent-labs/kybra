use quote::{format_ident, quote};

use crate::generators::stable_b_tree_map::StableBTreeMapNode;

pub fn generate_stable_b_tree_map_get(
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> proc_macro2::TokenStream {
    let match_arms = generate_match_arms(stable_b_tree_map_nodes);

    quote! {
        #[pymethod]
        fn _kybra_stable_b_tree_map_get(&self, memory_id_py_object_ref: PyObjectRef, key_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();

            match memory_id {
                #(#match_arms),*,
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
            let map_name_ident =
                format_ident!("STABLE_B_TREE_MAP_{}", stable_b_tree_map_node.memory_id);

            quote! {
                #memory_id => #map_name_ident.with(|p| p.borrow().get(&key_py_object_ref.try_from_vm_value(vm).unwrap())).try_into_vm_value(vm).unwrap()
            }
        })
        .collect()
}
