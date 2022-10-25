use rustpython_parser::ast::{Arguments, ExprKind, Located, StmtKind};

use crate::cdk_act::{
    traits::CanisterMethodBuilder, ActCanisterMethod, CanisterMethodType, RequestType,
};

pub struct KybraFunctionDef {
    pub name: String,
    pub args: Box<Arguments>,
    pub body: Vec<Located<StmtKind>>,
    pub decorator_list: Vec<Located<ExprKind>>,
    pub returns: Option<Box<Located<ExprKind>>>,
    pub type_comment: Option<String>,
}

impl KybraFunctionDef {
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
                ExprKind::Name { id, .. } => id == decorator_name,
                _ => false,
            })
    }
}

impl CanisterMethodBuilder for KybraFunctionDef {
    fn build_canister_method_node(&self, request_type: &RequestType) -> ActCanisterMethod {
        match request_type {
            RequestType::Query => todo!(),
            RequestType::Update => todo!(),
        }
    }

    fn build_params(&self) -> Vec<crate::cdk_act::nodes::ActFnParam> {
        todo!()
    }

    fn build_return_type(&self) -> crate::cdk_act::nodes::data_type_nodes::ActDataType {
        todo!()
    }
}
