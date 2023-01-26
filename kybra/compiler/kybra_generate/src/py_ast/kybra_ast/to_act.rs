use cdk_framework::{AbstractCanisterTree, ActCanisterMethod, ActDataType, ToAct};

use super::KybraAst;
use crate::generators::{
    header,
    vm_value_conversion::{try_from_vm_value, try_into_vm_value},
};

impl ToAct for KybraAst {
    fn to_act(&self) -> AbstractCanisterTree {
        let header = header::generate();

        let query_methods: Vec<ActCanisterMethod> = self
            .canister_methods
            .iter()
            .filter(|method| match method {
                ActCanisterMethod::QueryMethod { .. } => true,
                ActCanisterMethod::UpdateMethod(_) => false,
            })
            .cloned()
            .collect();
        let update_methods: Vec<ActCanisterMethod> = self
            .canister_methods
            .iter()
            .filter(|method| match method {
                ActCanisterMethod::QueryMethod { .. } => false,
                ActCanisterMethod::UpdateMethod(_) => true,
            })
            .cloned()
            .collect();

        let arrays = filter_by_variant(&self.canister_types, "Array");
        let funcs = filter_by_variant(&self.canister_types, "Func");
        let options = filter_by_variant(&self.canister_types, "Option");
        let primitives = filter_by_variant(&self.canister_types, "Primitive");
        let records = filter_by_variant(&self.canister_types, "Record");
        let tuples = filter_by_variant(&self.canister_types, "Tuple");
        let type_refs = filter_by_variant(&self.canister_types, "TypeRef");
        let variants = filter_by_variant(&self.canister_types, "Variant");

        let heartbeat_method = self.heartbeat.clone();
        let init_method = self.init_method.clone();
        let inspect_message_method = self.inspect_method.clone();
        let post_upgrade_method = self.post_upgrade.clone();
        let pre_upgrade_method = self.pre_upgrade.clone();

        let external_canisters = self.external_canisters.clone();

        let try_into_vm_value_impls = try_into_vm_value::generate_try_into_vm_value_impls();
        let try_from_vm_value_impls = try_from_vm_value::generate_try_from_vm_value_impls();

        AbstractCanisterTree {
            cdk_name: "kybra".to_string(),
            body: self.rust_code.clone(),
            update_methods,
            query_methods,
            heartbeat_method,
            init_method,
            inspect_message_method,
            post_upgrade_method,
            pre_upgrade_method,
            arrays,
            funcs,
            options,
            primitives,
            records,
            try_from_vm_value_impls,
            try_into_vm_value_impls,
            tuples,
            type_refs,
            variants,
            external_canisters,
            keywords: crate::get_python_keywords(),
            header,
        }
    }
}

fn filter_by_variant(types: &Vec<ActDataType>, variant: &str) -> Vec<ActDataType> {
    types
        .iter()
        .filter(|act| match act {
            ActDataType::Array(_) => variant == "Array",
            ActDataType::Func(_) => variant == "Func",
            ActDataType::Option(_) => variant == "Option",
            ActDataType::Primitive(_) => variant == "Primitive",
            ActDataType::Record(_) => variant == "Record",
            ActDataType::Tuple(_) => variant == "Tuple",
            ActDataType::TypeRef(_) => variant == "TypeRef",
            ActDataType::Variant(_) => variant == "Variant",
        })
        .cloned()
        .collect()
}
