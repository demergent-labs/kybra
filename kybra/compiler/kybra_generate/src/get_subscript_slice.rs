use rustpython_parser::ast::{ExprKind, Located};

use crate::{get_name::HasName, source_map::SourceMapped};

impl SourceMapped<&Located<ExprKind>> {
    pub fn get_subscript_slice_for(&self, type_name: &str) -> Option<&Located<ExprKind>> {
        if let ExprKind::Subscript { value, slice, .. } = &self.node {
            if value.get_name() == Some(type_name) {
                return Some(slice.as_ref());
            }
        }
        None
    }
}
