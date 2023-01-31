use cdk_framework::{
    act::{
        node::canister_method::{QueryMethod, UpdateMethod},
        CanisterMethods, DataTypes,
    },
    AbstractCanisterTree, ToAct,
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

        let arrays = self
            .canister_types
            .iter()
            .filter_map(|array| array.as_array().cloned())
            .collect();
        let funcs = self
            .canister_types
            .iter()
            .filter_map(|func| func.as_func().cloned())
            .collect();
        let options = self
            .canister_types
            .iter()
            .filter_map(|option| option.as_option().cloned())
            .collect();
        let primitives = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_primitive().cloned())
            .collect();
        let records = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_record().cloned())
            .collect();
        let tuples = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_tuple().cloned())
            .collect();
        let type_aliases = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_new_type_alias().cloned())
            .collect();
        let variants = self
            .canister_types
            .iter()
            .filter_map(|canister_type| canister_type.as_variant().cloned())
            .collect();

        let heartbeat_method = self.heartbeat.clone();
        let init_method = self.init_method.clone();
        let inspect_message_method = self.inspect_method.clone();
        let post_upgrade_method = self.post_upgrade.clone();
        let pre_upgrade_method = self.pre_upgrade.clone();

        let data_types = DataTypes {
            arrays,
            funcs,
            options,
            primitives,
            records,
            tuples,
            type_aliases,
            variants,
        };

        let canister_methods = CanisterMethods {
            heartbeat_method,
            init_method,
            inspect_message_method,
            post_upgrade_method,
            pre_upgrade_method,
            query_methods,
            update_methods,
        };

        let external_canisters = self.external_canisters.clone();

        let try_into_vm_value_impls = try_into_vm_value_impls::generate();
        let try_from_vm_value_impls = try_from_vm_value_impls::generate();

        AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            body: self.rust_code.clone(),
            try_from_vm_value_impls,
            try_into_vm_value_impls,
            external_canisters,
            keywords: crate::get_python_keywords(),
            header,
            function_guards: self.function_guards.iter().cloned().collect(),
            canister_methods,
            data_types,
        }
    }
}
