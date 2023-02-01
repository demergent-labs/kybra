use cdk_framework::{
    nodes::{
        ActCanisterMethod, ActExternalCanister, ActFunctionGuard, ActHeartbeatMethod,
        ActInitMethod, ActInspectMessageMethod, ActPostUpgradeMethod, ActPreUpgradeMethod,
    },
    ActDataType,
};
use proc_macro2::TokenStream;

mod to_act;

pub struct KybraAst {
    pub external_canisters: Vec<ActExternalCanister>,
    pub canister_methods: Vec<ActCanisterMethod>,
    pub function_guards: Vec<ActFunctionGuard>,
    pub canister_types: Vec<ActDataType>,
    pub init_method: ActInitMethod,
    pub inspect_method: Option<ActInspectMessageMethod>,
    pub pre_upgrade: ActPreUpgradeMethod,
    pub post_upgrade: ActPostUpgradeMethod,
    pub heartbeat: Option<ActHeartbeatMethod>,
    pub rust_code: TokenStream,
}

pub trait ToKybraAst {
    fn build_canister_method_act_nodes(&self) -> Vec<ActCanisterMethod>;
    fn get_act_data_type_nodes(&self) -> Vec<ActDataType>;
}
