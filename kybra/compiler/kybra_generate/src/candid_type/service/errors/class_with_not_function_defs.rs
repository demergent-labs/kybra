use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ClassWithNotFunctionDefs {
    pub class_name: String,
    pub location: Location,
}

impl ClassWithNotFunctionDefs {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>, class_name: &str) -> Error {
        Self {
            class_name: class_name.to_string(),
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<ClassWithNotFunctionDefs> for Error {
    fn from(value: ClassWithNotFunctionDefs) -> Self {
        Self::ClassWithNotFunctionDefs(value)
    }
}

impl std::fmt::Display for ClassWithNotFunctionDefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
        CompilerOutput {
            title: format!("class \"{}\" should only contain function definitions. Please remove everything else.", self.class_name),
            location: self.location.clone(),
            annotation: "".to_string(),
            suggestion: None,
        }
                .to_string(AnnotationType::Error),
    )
    }
}
