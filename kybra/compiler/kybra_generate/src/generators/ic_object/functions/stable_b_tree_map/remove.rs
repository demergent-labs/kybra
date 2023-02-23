use quote::quote;

use crate::{generators::stable_b_tree_map, kybra_ast::node::StableBTreeMapNode};

pub fn generate(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> proc_macro2::TokenStream {
    let match_arms = generate_match_arms(stable_b_tree_map_nodes);

    quote! {
        #[pymethod]
        fn _kybra_stable_b_tree_map_remove(&self, memory_id_py_object_ref: PyObjectRef, key_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();

            match memory_id {
                #(#match_arms),*
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

            let (key_wrapper_type_name, _) = stable_b_tree_map::wrapper_type::generate(&stable_b_tree_map_node.key_type, memory_id, "Key");

            quote! {
                #memory_id => {
                    match #map_name_ident.with(|p| p.borrow_mut().remove(&#key_wrapper_type_name(key_py_object_ref.try_from_vm_value(vm).unwrap()))) {
                        Some(value) => value.0.try_into_vm_value(vm).unwrap(),
                        None => vm.ctx.none()
                    }
                }
            }
        })
        .collect()
}
