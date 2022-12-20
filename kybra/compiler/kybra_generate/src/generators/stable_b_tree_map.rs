use quote::{format_ident, quote};

pub struct StableBTreeMapNode {
    pub memory_id: u8,
    pub key_type: proc_macro2::TokenStream,
    pub value_type: proc_macro2::TokenStream,
    pub max_key_size: u32,
    pub max_value_size: u32,
}

pub fn generate_stable_b_tree_map(
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> proc_macro2::TokenStream {
    let stable_b_tree_maps = generate_global_stable_b_tree_maps(stable_b_tree_map_nodes);

    quote! {
        use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
        use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
        use std::cell::RefCell;

        // TODO prefix everything

        type Memory = VirtualMemory<DefaultMemoryImpl>;

        thread_local! {
            static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

            #(#stable_b_tree_maps)*
        }
    }
}

fn generate_global_stable_b_tree_maps(
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> Vec<proc_macro2::TokenStream> {
    stable_b_tree_map_nodes
        .iter()
        .map(|stable_b_tree_map_node| {
            let map_name_ident =
                format_ident!("STABLE_B_TREE_MAP_{}", stable_b_tree_map_node.memory_id);
            let memory_id = stable_b_tree_map_node.memory_id;
            let key_type = &stable_b_tree_map_node.key_type;
            let value_type = &stable_b_tree_map_node.value_type;

            quote! {
                static #map_name_ident: RefCell<StableBTreeMap<Memory, #key_type, #value_type>> = RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(#memory_id))),));
            }
        })
        .collect()
}
