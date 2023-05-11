use cdk_framework::{
    act::{node::Param, CandidTypes, CanisterMethods, VmValueConversion},
    traits::CollectResults,
    AbstractCanisterTree,
};
use proc_macro2::TokenStream;

use super::PyAst;
use crate::{
    body, header, keywords,
    vm_value_conversion::{try_from_vm_value_impls, try_into_vm_value_impls},
    Error,
};

impl PyAst {
    pub fn to_act(&self) -> Result<AbstractCanisterTree, Vec<Error>> {
        let (candid_types, canister_methods_tuple, guard_functions, stable_b_tree_map_nodes) = (
            self.create_candid_types(),
            self.create_canister_methods(),
            self.build_guard_functions(),
            self.build_stable_b_tree_map_nodes(),
        )
            .collect_results()?;

        let (
            canister_methods,
            init_params,
            call_to_init_py_function,
            call_to_post_upgrade_py_function,
        ) = canister_methods_tuple;

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
                &candid_types.services,
                &stable_b_tree_map_nodes,
                &self.entry_module_name,
                &init_params,
                call_to_init_py_function,
                call_to_post_upgrade_py_function,
            ),
            candid_types,
            canister_methods,
            guard_functions,
            vm_value_conversion,
            keywords: keywords::get_python_keywords(),
        })
    }

    pub fn create_candid_types(&self) -> Result<CandidTypes, Vec<Error>> {
        let (funcs, records, services, tuples, type_aliases, variants) = (
            self.build_funcs(),
            self.build_records(),
            self.build_services(),
            self.build_tuples(),
            self.build_type_aliases(),
            self.build_variants(),
        )
            .collect_results()?;

        Ok(CandidTypes {
            funcs,
            records,
            services,
            tuples,
            type_aliases,
            variants,
        })
    }

    pub fn create_canister_methods(
        &self,
    ) -> Result<(CanisterMethods, Vec<Param>, TokenStream, TokenStream), Vec<Error>> {
        let (
            heartbeat_method,
            (init_method, init_params, call_to_init_py_function),
            inspect_message_method,
            (post_upgrade_method, call_to_post_upgrade_py_function),
            pre_upgrade_method,
            query_methods,
            update_methods,
        ) = (
            self.build_heartbeat_method(),
            self.build_init_method(),
            self.build_inspect_method(),
            self.build_post_upgrade_method(),
            self.build_pre_upgrade_method(),
            self.build_query_methods(),
            self.build_update_methods(),
        )
            .collect_results()?;

        Ok((
            CanisterMethods {
                heartbeat_method,
                init_method: Some(init_method),
                inspect_message_method,
                post_upgrade_method: Some(post_upgrade_method),
                pre_upgrade_method,
                query_methods,
                update_methods,
            },
            init_params,
            call_to_init_py_function,
            call_to_post_upgrade_py_function,
        ))
    }
}
