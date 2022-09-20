use super::nodes::{
    alias::AliasActNode, array::ArrayActNode,
    cross_canister_function::CrossCanisterFunctionActNode, func::FuncActNode,
    heartbeat_method::HeartbeatMethodActNode, init_method::InitMethodActNode,
    inspect_message_method::InspectMessageMethodActNode, option::OptionActNode,
    post_upgrade_method::PostUpgradeMethodActNode, pre_upgrade_method::PreUpgradeMethodActNode,
    primitive::PrimitiveActNode, query_method::QueryMethodActNode, record::RecordActNode,
    tuple::TupleActNode, update_method::UpdateMethodActNode, variant::VariantActNode,
};
use quote::quote;

// TODO I'm not sure if we should have the concrete ActNode types here or just the ActNode enum itself
pub struct AbstractCanisterTree {
    pub aliases: Vec<AliasActNode>, // TODO I am not sure we need this??
    pub arrays: Vec<ArrayActNode>,
    pub cross_canister_functions: Vec<CrossCanisterFunctionActNode>,
    pub funcs: Vec<FuncActNode>,
    pub heartbeat_method: HeartbeatMethodActNode,
    pub init_method: InitMethodActNode,
    pub inspect_message_method: InspectMessageMethodActNode,
    pub options: Vec<OptionActNode>,
    pub post_upgrade_method: PostUpgradeMethodActNode,
    pub pre_upgrade_method: PreUpgradeMethodActNode,
    pub primitives: Vec<PrimitiveActNode>,
    pub query_methods: Vec<QueryMethodActNode>,
    pub records: Vec<RecordActNode>,
    pub tuples: Vec<TupleActNode>,
    pub update_methods: Vec<UpdateMethodActNode>,
    pub variants: Vec<VariantActNode>,
}

trait Act {
    fn to_token_stream(self) -> proc_macro2::TokenStream;
}

impl Act for Vec<AliasActNode> {
    fn to_token_stream(self) -> proc_macro2::TokenStream {
        quote! {}
    }
}

impl Act for AbstractCanisterTree {
    fn to_token_stream(self) -> proc_macro2::TokenStream {
        // TODO turn each field in the ACT into its token stream
        // let aliases_token_stream = self.aliases.to_token_stream();

        quote! {}
    }
}
