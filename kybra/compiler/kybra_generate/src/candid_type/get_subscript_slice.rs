use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::source_map::SourceMapped;

pub struct PythonClass<'a> {
    pub name: &'a String,
    pub body: &'a Vec<Located<StmtKind>>,
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn get_subscript_slice_for(&self, type_id: &str) -> Option<&Located<ExprKind>> {
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => match &value.node {
                ExprKind::Name { id, .. } => match id == type_id {
                    true => Some(slice.as_ref()),
                    false => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn get_child_class_of(&self, class_id: &str) -> Option<PythonClass> {
        match &self.node {
            StmtKind::ClassDef {
                bases, name, body, ..
            } => {
                if bases.iter().any(|base| match &base.node {
                    ExprKind::Name { id, .. } => id == class_id,
                    _ => false,
                }) {
                    return Some(PythonClass { name, body });
                }
                None
            }
            _ => None,
        }
    }
}
