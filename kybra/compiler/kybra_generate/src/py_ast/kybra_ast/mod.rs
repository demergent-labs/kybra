mod to_act;

use crate::cdk_act::{
    nodes::{ActInitMethod, ActPostUpgradeMethod},
    ActCanisterMethod, ActDataType,
};

pub struct KybraAst {
    pub canister_methods: Vec<ActCanisterMethod>,
    pub canister_types: Vec<ActDataType>,
    pub init_method: ActInitMethod,
    pub post_upgrade: ActPostUpgradeMethod,
}

pub trait ToKybraAst {
    fn build_canister_method_act_nodes(&self) -> Vec<ActCanisterMethod>;
    fn get_act_data_type_nodes(&self) -> Vec<ActDataType>;
}
