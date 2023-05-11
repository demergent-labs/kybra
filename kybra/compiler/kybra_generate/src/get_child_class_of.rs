use rustpython_parser::ast::{Located, StmtKind};

use crate::{get_name::HasName, source_map::SourceMapped};

pub struct PythonClass<'a> {
    pub name: &'a String,
    pub body: &'a Vec<Located<StmtKind>>,
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn get_child_class_of(&self, class_id: &str) -> Option<PythonClass> {
        if let StmtKind::ClassDef {
            name, bases, body, ..
        } = &self.node
        {
            if bases.iter().any(|base| base.get_name() == Some(class_id)) {
                return Some(PythonClass { name, body });
            }
        }
        None
    }
}
