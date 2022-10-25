use std::collections::{HashMap, HashSet};

use rustpython_parser::ast::{Arguments, ExprKind, Located, StmtKind};

use crate::cdk_act::{nodes::data_type_nodes::ActDataType, CanisterMethodType};

pub struct KybraTypeAliasDecl<U = ()> {
    pub name: String,
    pub args: Box<Arguments<U>>,
    pub body: Vec<Located<StmtKind<U>, U>>,
    pub decorator_list: Vec<Located<ExprKind<U>, U>>,
    pub returns: Option<Box<Located<ExprKind<U>, U>>>,
    pub type_comment: Option<String>,
}

impl KybraTypeAliasDecl {
    pub fn is_canister_method_type(&self, canister_method_type: &CanisterMethodType) -> bool {
        self.is_decorator_same_as(match canister_method_type {
            CanisterMethodType::Heartbeat => "heartbeat",
            CanisterMethodType::Init => "init",
            CanisterMethodType::InspectMessage => "inspect_message",
            CanisterMethodType::PostUpgrade => "post_upgrade",
            CanisterMethodType::PreUpgrade => "pre_upgrade",
            CanisterMethodType::Query => "query",
            CanisterMethodType::Update => "update",
        })
    }

    fn is_decorator_same_as(&self, decorator_name: &str) -> bool {
        self.decorator_list
            .iter()
            .any(|expr_kind| match &expr_kind.node {
                ExprKind::Name { id, .. } => match &id[..] {
                    name => true,
                    _ => false,
                },
                _ => false,
            })
    }
}

pub trait KybraTypeAliasListHelperMethods {
    fn generate_type_alias_lookup(&self) -> HashMap<String, KybraTypeAliasDecl>;
    fn build_type_alias_acts(&self, type_names: &HashSet<String>) -> Vec<ActDataType>;
    fn get_azle_type_aliases_by_type_ref_name(
        &self,
        type_ref_name: &str,
    ) -> Vec<KybraTypeAliasDecl>;
}

impl KybraTypeAliasListHelperMethods for Vec<KybraTypeAliasDecl> {
    fn generate_type_alias_lookup(&self) -> HashMap<String, KybraTypeAliasDecl> {
        todo!()
    }

    fn build_type_alias_acts(&self, type_names: &HashSet<String>) -> Vec<ActDataType> {
        todo!()
    }

    fn get_azle_type_aliases_by_type_ref_name(
        &self,
        type_ref_name: &str,
    ) -> Vec<KybraTypeAliasDecl> {
        todo!()
    }
}
