use cdk_framework::{
    act::{
        node::canister_method::{QueryMethod, UpdateMethod},
        CanisterMethods, DataTypes, ToAct,
    },
    AbstractCanisterTree,
};

use super::KybraAst;
use crate::generators::{
    header,
    vm_value_conversion::{try_from_vm_value_impls, try_into_vm_value_impls},
};

impl ToAct for KybraAst {
    fn to_act(&self) -> AbstractCanisterTree {
        let header = header::generate();

        let query_methods: Vec<QueryMethod> = self.query_methods.iter().cloned().collect();
        let update_methods: Vec<UpdateMethod> = self.update_methods.iter().cloned().collect();

        let heartbeat_method = self.heartbeat.clone();
        let init_method = self.init_method.clone();
        let inspect_message_method = self.inspect_method.clone();
        let post_upgrade_method = self.post_upgrade.clone();
        let pre_upgrade_method = Some(self.pre_upgrade.clone());

        let canister_methods = CanisterMethods {
            heartbeat_method,
            init_method,
            inspect_message_method,
            post_upgrade_method,
            pre_upgrade_method,
            query_methods,
            update_methods,
        };

        let funcs = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_func())
            .cloned()
            .collect();
        let records = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_record())
            .cloned()
            .collect();
        let tuples = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_tuple())
            .cloned()
            .collect();
        let type_aliases = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_type_alias())
            .cloned()
            .collect();
        let variants = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_variant())
            .cloned()
            .collect();

        let data_types = DataTypes {
            funcs,
            records,
            tuples,
            type_aliases,
            variants,
        };

        let external_canisters = self.external_canisters.clone();

        let try_into_vm_value_impls = try_into_vm_value_impls::generate();
        let try_from_vm_value_impls = try_from_vm_value_impls::generate();

        AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            body: self.rust_code.clone(),
            canister_methods,
            data_types,
            try_from_vm_value_impls,
            try_into_vm_value_impls,
            external_canisters,
            keywords: crate::get_python_keywords(),
            header,
            guard_functions: self.function_guards.iter().cloned().collect(),
        }
    }
}
