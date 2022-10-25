use crate::{
    cdk_act::{nodes::ActCanisterMethod, traits::CanisterMethodBuilder, RequestType},
    py_ast::KybraFunctionDef,
};

pub fn build_canister_method_nodes(
    function_defs: &Vec<KybraFunctionDef>,
    request_type: RequestType,
) -> Vec<ActCanisterMethod> {
    function_defs.iter().fold(vec![], |acc, function_def| {
        let canister_method_node = function_def.build_canister_method_node(&request_type);
        vec![acc, vec![canister_method_node]].concat()
    })
}
