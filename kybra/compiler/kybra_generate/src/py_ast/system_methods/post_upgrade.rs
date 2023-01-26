use cdk_framework::{
    nodes::{ActExternalCanister, ActPostUpgradeMethod},
    ActCanisterMethod, CanisterMethodType,
};

use crate::{
    generators::{canister_methods::post_upgrade, ic_object},
    py_ast::{kybra_types::StableBTreeMapNode, PyAst},
};

impl PyAst<'_> {
    pub fn build_post_upgrade_method(
        &self,
        canister_methods: &Vec<ActCanisterMethod>,
        external_canisters: &Vec<ActExternalCanister>,
        stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
    ) -> ActPostUpgradeMethod {
        // TODO: Abstract IC Object generation out to a standalone function
        // in the generated code, not just in our code.
        let ic_object = ic_object::generate(
            canister_methods,
            external_canisters,
            stable_b_tree_map_nodes,
        );

        let post_upgrade_function_defs =
            self.get_function_def_of_type(CanisterMethodType::PostUpgrade);

        if post_upgrade_function_defs.len() > 1 {
            todo!();
        }

        let post_upgrade_function_def_option = post_upgrade_function_defs.get(0);

        let params = match &post_upgrade_function_def_option {
            Some(post_upgrade_function_def) => post_upgrade_function_def.build_params(),
            None => vec![],
        };

        let body = post_upgrade::generate_post_upgrade_method_body(
            post_upgrade_function_def_option,
            &self.entry_module_name,
            ic_object,
        );
        ActPostUpgradeMethod { params, body }
    }
}
