use cdk_framework::act::node::{
    canister_method::{
        HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
        QueryMethod, UpdateMethod,
    },
    DataType, ExternalCanister, FunctionGuard,
};
use proc_macro2::TokenStream;

mod to_act;

pub struct KybraAst {
    pub external_canisters: Vec<ExternalCanister>,
    pub query_methods: Vec<QueryMethod>,
    pub update_methods: Vec<UpdateMethod>,
    pub canister_types: Vec<DataType>,
    pub init_method: InitMethod,
    pub inspect_method: Option<InspectMessageMethod>,
    pub pre_upgrade: PreUpgradeMethod,
    pub post_upgrade: PostUpgradeMethod,
    pub heartbeat: Option<HeartbeatMethod>,
    pub rust_code: TokenStream,
    pub function_guards: Vec<FunctionGuard>,
}

pub trait ToKybraAst {
    fn build_update_method_act_nodes(&self) -> Vec<UpdateMethod>;
    fn build_query_method_act_nodes(&self) -> Vec<QueryMethod>;
    fn get_act_data_type_nodes(&self) -> Vec<DataType>;
}
