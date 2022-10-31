mod errors;

use crate::{
    cdk_act::{nodes::ActFnParam, ToActDataType},
    py_ast::{kybra_types::KybraExpr, what_is_it::WhatIsIt},
};

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn build_params(&self) -> Vec<ActFnParam> {
        self.stmt_kind.what_is_it();
        match &self.stmt_kind.node {
            rustpython_parser::ast::StmtKind::FunctionDef { args, .. } => {
                for arg in &args.args {
                    arg.what_is_it()
                }
                args.args
                    .iter()
                    .map(|arg| {
                        let data_type = match &arg.node.annotation {
                            Some(annotation) => KybraExpr {
                                located_expr: &annotation,
                                source_map: self.source_map,
                            }
                            .to_act_data_type(&None),
                            None => panic!("{}", self.missing_type_annotation_error()),
                        };
                        ActFnParam {
                            name: arg.node.arg.clone(),
                            data_type,
                        }
                    })
                    .collect()
            }
            _ => panic!("{}", self.not_an_init_method_error()),
        }
    }
}
