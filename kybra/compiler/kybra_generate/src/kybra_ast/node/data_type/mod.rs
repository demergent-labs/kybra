use cdk_framework::{act::node::DataType, ToDataType};
use rustpython_parser::ast::{ExprKind, Located};

use crate::source_map::SourceMapped;

pub mod array;
pub mod func;
pub mod opt;
pub mod primitive;
pub mod record;
pub mod tuple;
pub mod type_alias;
pub mod type_ref;
pub mod variant;

impl ToDataType for SourceMapped<'_, Located<ExprKind>> {
    fn to_data_type(&self) -> DataType {
        if self.is_primitive() {
            match self.to_primitive() {
                Ok(primitive) => DataType::Primitive(primitive),
                Err(error) => panic!("{}", error),
            }
        } else {
            panic!();
        }
    }
}
