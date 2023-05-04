use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ClassMustHaveMethods {
    pub class_name: String,
    pub location: Location,
}

impl ClassMustHaveMethods {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>, class_name: &str) -> Error {
        Self {
            class_name: class_name.to_string(),
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<ClassMustHaveMethods> for Error {
    fn from(value: ClassMustHaveMethods) -> Self {
        Self::ClassMustHaveMethods(value)
    }
}

impl Display for ClassMustHaveMethods {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            CompilerOutput {
                title: format!("class \"{}\" doesn't have any methods. External canisters are required to expose at least one method.", self.class_name),
                location: self.location.clone(),
                annotation: "".to_string(),
                suggestion: None,
            }.to_string(AnnotationType::Error),
        )
    }
}
