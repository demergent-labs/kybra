use cdk_framework::{
    act::{CandidTypes, CanisterMethods, VmValueConversion},
    AbstractCanisterTree,
};

use super::PyAst;
use crate::{
    errors::KybraResult,
    generators::{
        body, header,
        vm_value_conversion::{try_from_vm_value_impls, try_into_vm_value_impls},
    },
};

impl PyAst {
    pub fn to_act(&self) -> KybraResult<AbstractCanisterTree> {
        let stable_b_tree_map_nodes = self.build_stable_b_tree_map_nodes()?;
        let external_canisters = self.build_external_canisters()?;

        let canister_methods = CanisterMethods {
            heartbeat_method: self.build_heartbeat_method()?,
            init_method: self.build_init_method()?,
            inspect_message_method: self.build_inspect_method()?,
            post_upgrade_method: self.build_post_upgrade_method()?,
            pre_upgrade_method: self.build_pre_upgrade_method()?,
            query_methods: self.build_query_methods()?,
            update_methods: self.build_update_methods()?,
        };

        let candid_types = CandidTypes {
            funcs: self.build_funcs()?,
            records: self.build_records()?,
            tuples: self.build_tuples()?,
            type_aliases: self.build_type_aliases()?,
            variants: self.build_variants()?,
        };

        let vm_value_conversion = VmValueConversion {
            try_from_vm_value_impls: try_into_vm_value_impls::generate(),
            try_into_vm_value_impls: try_from_vm_value_impls::generate(),
        };

        Ok(AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            header: header::generate(),
            body: body::generate(
                &canister_methods.update_methods,
                &canister_methods.query_methods,
                &external_canisters,
                &stable_b_tree_map_nodes,
            ),
            candid_types,
            canister_methods,
            external_canisters,
            guard_functions: self.build_guard_functions()?,
            vm_value_conversion,
            keywords: crate::get_python_keywords(),
        })
    }
}
