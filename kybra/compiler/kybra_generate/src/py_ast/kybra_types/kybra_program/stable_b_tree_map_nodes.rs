use cdk_framework::act::node::DataType;

use super::KybraProgram;

// TODO all variables should be called stable_b_tree_map_nodes
#[derive(Clone)]
pub struct StableBTreeMapNode {
    pub memory_id: u8,
    pub key_type: DataType,
    pub value_type: DataType,
    pub max_key_size: u32,
    pub max_value_size: u32,
}

impl KybraProgram {
    pub fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        self.get_kybra_stable_b_tree_node_stmts()
            .iter()
            .map(|kybra_stmt| kybra_stmt.as_stable_b_tree_map_node())
            .collect()
    }
}
