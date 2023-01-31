use cdk_framework::act::node::canister_methods::{
    ActCanisterMethod, InitMethod, PostUpgradeMethod, PreUpgradeMethod, UpdateMethod,
};
use cdk_framework::act::node::data_types::primitive::ActPrimitiveLit;
use cdk_framework::act::node::data_types::{
    ActArray, ActFunc, ActOption, ActPrimitive, ActRecord, ActTuple, ActTypeRef, ActVariant,
};
use cdk_framework::act::node::{ActDataType, ActNode};
use cdk_framework::{AbstractCanisterTree, ToAct, ToActDataType};
use quote::quote;
use rustpython_parser::ast::{Located, StmtKind};
use std::fmt;
use std::{error::Error, vec};

struct KybraAstNew {
    pub query_method_ast_nodes: Vec<KybraQueryMethodAstNode>,
    pub update_method_ast_nodes: Vec<KybraUpdateMethodAstNode>,
}

struct KybraQueryMethodAstNode {
    pub name: String,
    pub py_ast_node: Located<StmtKind>,
}

struct KybraUpdateMethodAstNode {
    pub name: String,
    pub py_ast_node: Located<StmtKind>,
}

#[derive(Clone)]
struct ActDataTypes {
    pub arrays: Vec<ActArray>,
    pub funcs: Vec<ActFunc>,
    pub options: Vec<ActOption>,
    pub primitives: Vec<ActPrimitive>,
    pub records: Vec<ActRecord>,
    pub tuples: Vec<ActTuple>,
    pub type_refs: Vec<ActTypeRef>,
    pub variants: Vec<ActVariant>,
}

#[derive(Debug, Clone)]
enum ActError {}

impl Error for ActError {}

impl fmt::Display for ActError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

trait KybraAstNode {
    fn to_everything(&self) -> Result<(ActNode, ActDataTypes), ActError> {
        let act = self.to_act().unwrap();
        let data_types = self.get_act_data_types().unwrap();
        Ok((act, data_types))
    }
    fn to_act(&self) -> Result<ActNode, ActError>;
    fn get_act_data_types(&self) -> Result<ActDataTypes, ActError> {
        Ok(ActDataTypes {
            arrays: vec![],
            funcs: vec![],
            options: vec![],
            primitives: vec![],
            records: vec![],
            tuples: vec![],
            type_refs: vec![],
            variants: vec![],
        })
    }
}

impl KybraAstNode for KybraUpdateMethodAstNode {
    fn to_act(&self) -> Result<ActNode, ActError> {
        Ok(ActNode::CanisterMethod(ActCanisterMethod::Update(
            UpdateMethod {
                body: quote!(),
                params: vec![],
                is_manual: false,
                is_async: false,
                name: self.name.clone(),
                return_type: ActPrimitiveLit::Bool.to_act_data_type(&None),
                cdk_name: "kybra".to_string(),
                function_guard_name: None,
            },
        )))
    }
}
impl KybraAstNode for KybraQueryMethodAstNode {
    fn to_act(&self) -> Result<ActNode, ActError> {
        Ok(ActNode::CanisterMethod(ActCanisterMethod::Update(
            UpdateMethod {
                body: quote!(),
                params: vec![],
                is_manual: false,
                is_async: false,
                name: self.name.clone(),
                return_type: ActPrimitiveLit::Bool.to_act_data_type(&None),
                cdk_name: "kybra".to_string(),
                function_guard_name: None,
            },
        )))
    }
}

trait ToAlmostAct {
    fn to_almost_act(&self) -> (Vec<ActNode>, Vec<ActDataTypes>, Vec<ActError>);
}

impl<T> ToAlmostAct for Vec<T>
where
    T: KybraAstNode,
{
    fn to_almost_act(&self) -> (Vec<ActNode>, Vec<ActDataTypes>, Vec<ActError>) {
        let results: Vec<Result<(ActNode, ActDataTypes), ActError>> =
            self.iter().map(|update| update.to_everything()).collect();

        let almost_acts: Vec<(ActNode, ActDataTypes)> = results
            .clone()
            .iter()
            .filter_map(|result| result.clone().ok())
            .collect();

        let act_nodes = almost_acts
            .iter()
            .map(|almost_act| almost_act.0.clone())
            .collect();

        let act_subsets = almost_acts
            .iter()
            .map(|almost_act| almost_act.1.clone())
            .collect();

        let act_errors: Vec<ActError> = results
            .iter()
            .filter_map(|result| result.clone().err())
            .collect();

        (act_nodes, act_subsets, act_errors)
    }
}

impl ToAct for KybraAstNew {
    fn to_act(&self) -> cdk_framework::AbstractCanisterTree {
        let (update_method_act_nodes, update_method_act_data_types, update_method_errors) =
            self.update_method_ast_nodes.to_almost_act();
        let (query_method_act_nodes, query_method_act_data_types, query_method_errors) =
            self.query_method_ast_nodes.to_almost_act();
        // let query_method_act_nodes = query_method_act_nodes
        //     .iter()
        //     .map(|query_method| query_method.as_query_method());

        let all_act_data_types = vec![update_method_act_data_types, query_method_act_data_types];

        let deduplicated = deduplicate(all_act_data_types);

        let pre_upgrade_method = PreUpgradeMethod { body: quote!() };
        let post_upgrade_method = PostUpgradeMethod {
            body: quote!(),
            params: vec![],
        };
        let init_method = InitMethod {
            params: vec![],
            body: quote!(),
        };

        AbstractCanisterTree {
            arrays: deduplicated.arrays,
            body: quote!(),
            cdk_name: "kybra".to_string(),
            external_canisters: vec![],
            funcs: deduplicated.funcs,
            header: quote!(),
            heartbeat_method: None,
            init_method,
            inspect_message_method: None,
            keywords: vec![],
            options: deduplicated.options,
            post_upgrade_method,
            pre_upgrade_method,
            primitives: deduplicated.primitives,
            query_methods: vec![],
            records: deduplicated.records,
            try_from_vm_value_impls: quote!(),
            try_into_vm_value_impls: quote!(),
            tuples: deduplicated.tuples,
            type_refs: deduplicated.type_refs,
            update_methods: vec![],
            variants: deduplicated.variants,
            function_guards: vec![],
        }
    }
}

fn deduplicate(all_things: Vec<Vec<ActDataTypes>>) -> ActDataTypes {
    // let empty = ActSubset { arrays: (), funcs: (), options: (), primitives: (), records: (), tuples: (), type_refs: (), variants: () }
    ActDataTypes {
        arrays: vec![],
        funcs: vec![],
        options: vec![],
        primitives: vec![],
        records: vec![],
        tuples: vec![],
        type_refs: vec![],
        variants: vec![],
    }
    // all_things.iter().fold(empty, f)
}