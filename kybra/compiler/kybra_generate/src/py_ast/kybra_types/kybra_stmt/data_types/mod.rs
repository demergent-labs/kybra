pub mod errors;
pub mod func;
pub mod records;
pub mod tuples;
pub mod type_alias;
pub mod variants;

use cdk_framework::act::node::DataType;
use rustpython_parser::ast::StmtKind;

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn build_act_data_type(&self) -> DataType {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { .. } => {
                if self.is_record() {
                    return self.as_record();
                } else if self.is_variant() {
                    return self.as_variant();
                }
                panic!("{}", self.invalid_class_error())
            }
            StmtKind::Assign { .. } => {
                if self.is_tuple() {
                    return self.as_tuple();
                } else if self.is_type_alias() {
                    return self.as_type_alias();
                }
                panic!("{}", self.invalid_assign_error());
            }
            StmtKind::AnnAssign { .. } => {
                if self.is_func() {
                    return self.as_func();
                } else if self.is_type_alias() {
                    return self.as_type_alias();
                }
                panic!("{}", self.invalid_annotation_assign_error());
            }
            _ => panic!("{}", self.unsupported_type_error()),
        }
    }
}
