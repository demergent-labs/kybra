use cdk_framework::{act::node::param::Param, ToDataType};
use proc_macro2::TokenStream;

use super::KybraStmt;
use crate::{generators::canister_methods, py_ast::kybra_types::KybraExpr};

mod errors;

impl KybraStmt<'_> {
    pub fn build_params(&self) -> Vec<Param> {
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { args, .. } => args
                .args
                .iter()
                .map(|arg| {
                    let data_type = match &arg.node.annotation {
                        Some(annotation) => KybraExpr {
                            located_expr: &annotation,
                            source_map: self.source_map.clone(),
                        }
                        .to_data_type(),
                        None => panic!("{}", self.missing_type_annotation_error()),
                    };
                    Param {
                        name: arg.node.arg.clone(),
                        type_: data_type,
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
