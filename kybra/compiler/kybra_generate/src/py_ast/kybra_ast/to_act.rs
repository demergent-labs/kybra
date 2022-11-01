use quote::quote;

use crate::{
    cdk_act::{
        nodes::ActPreUpgradeMethod, AbstractCanisterTree, ActCanisterMethod, ActDataType, ToAct,
    },
    generators::{
        async_result_handler::generate_async_result_handler,
        vm_value_conversion::{try_from_vm_value, try_into_vm_value},
    },
};

use super::KybraAst;

impl ToAct for KybraAst {
    fn to_act(&self) -> AbstractCanisterTree {
        let query_methods: Vec<ActCanisterMethod> = self
            .canister_methods
            .iter()
            .filter(|method| match method {
                ActCanisterMethod::QueryMethod(_) => true,
                ActCanisterMethod::UpdateMethod(_) => false,
            })
            .map(|method| method.clone())
            .collect();
        let update_methods: Vec<ActCanisterMethod> = self
            .canister_methods
            .iter()
            .filter(|method| match method {
                ActCanisterMethod::QueryMethod(_) => false,
                ActCanisterMethod::UpdateMethod(_) => true,
            })
            .map(|method| method.clone())
            .collect();

        let arrays: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Array(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let funcs: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Func(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let options: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Option(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let primitives: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Primitive(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let records: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Record(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let tuples: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Tuple(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let type_refs: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::TypeRef(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();
        let variants: Vec<ActDataType> = self
            .canister_types
            .iter()
            .filter(|act| match act {
                ActDataType::Variant(_) => true,
                _ => false,
            })
            .map(|act| act.clone())
            .collect();

        let heartbeat_method = None;
        let init_method = self.init_method.clone();
        let inspect_message_method = None;
        let post_upgrade_method = self.post_upgrade.clone();
        let pre_upgrade_method = ActPreUpgradeMethod { body: quote!() };

        let try_into_vm_value_impls = try_into_vm_value::generate_try_into_vm_value_impls();
        let try_from_vm_value_impls = try_from_vm_value::generate_try_from_vm_value_impls();

        let async_result_handler = generate_async_result_handler();
        let get_top_level_call_frame_fn = quote!();

        let cross_canister_call_functions = quote!();

        AbstractCanisterTree {
            update_methods,
            query_methods,
            heartbeat_method,
            init_method,
            inspect_message_method,
            post_upgrade_method,
            pre_upgrade_method,
            rust_code: quote! {
                #cross_canister_call_functions
                #async_result_handler
                #get_top_level_call_frame_fn
            },
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
        }
    }
}
