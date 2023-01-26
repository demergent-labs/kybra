use cdk_framework::{
    nodes::{ActExternalCanister, ActInitMethod},
    ActCanisterMethod, CanisterMethodType,
};

use crate::{
    generators::{canister_methods::init, ic_object},
    py_ast::{kybra_types::StableBTreeMapNode, PyAst},
};

impl PyAst<'_> {
    pub fn build_init_method(
        &self,
        canister_methods: &Vec<ActCanisterMethod>,
        external_canisters: &Vec<ActExternalCanister>,
        stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
    ) -> ActInitMethod {
        // TODO: Abstract IC Object generation out to a standalone function
        // in the generated code, not just in our code.
        let ic_object = ic_object::generate(
            canister_methods,
            external_canisters,
            stable_b_tree_map_nodes,
        );
        let init_function_defs = self.get_function_def_of_type(CanisterMethodType::Init);

        if init_function_defs.len() > 1 {
            todo!();
        }

        let init_function_def_option = init_function_defs.get(0);

        let params = match init_function_def_option {
            Some(init_function_def) => init_function_def.build_params(),
            None => vec![],
        };

        let body = init::generate_init_method_body(
            init_function_def_option,
            &self.entry_module_name,
            ic_object,
        );
        ActInitMethod { params, body }
    }
}
