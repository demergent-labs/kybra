use cdk_framework::{act::node::canister_method::FnParam, ToActDataType};
use proc_macro2::TokenStream;

use super::KybraStmt;
use crate::{generators::canister_methods, py_ast::kybra_types::KybraExpr};

mod errors;

impl KybraStmt<'_> {
    pub fn build_params(&self) -> Vec<FnParam> {
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { args, .. } => args
                .args
                .iter()
                .map(|arg| {
                    let data_type = match &arg.node.annotation {
                        Some(annotation) => KybraExpr {
                            located_expr: &annotation,
                            programs: self.programs,
                            source_map: self.source_map,
                        }
                        .to_act_data_type(&None),
                        None => panic!("{}", self.missing_type_annotation_error()),
                    };
                    FnParam {
                        name: arg.node.arg.clone(),
                        data_type,
                    }
                })
                .collect(),
            _ => panic!("{}", self.not_a_function_def_error()),
        }
    }

    pub fn get_function_name(&self) -> String {
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { name, .. } => name.clone(),
            _ => panic!("{}", self.not_a_function_def_error()),
        }
    }

    pub fn generate_call_to_py_function(&self) -> TokenStream {
        canister_methods::generate_call_to_py_function(self)
    }
}
