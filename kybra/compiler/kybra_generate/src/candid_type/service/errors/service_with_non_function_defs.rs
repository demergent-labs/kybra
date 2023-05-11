use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ServiceWithNonFunctionDefs {
    pub service_name: String,
    pub location: Location,
}

impl ServiceWithNonFunctionDefs {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>, class_name: &str) -> Error {
        Self {
            service_name: class_name.to_string(),
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<ServiceWithNonFunctionDefs> for Error {
    fn from(value: ServiceWithNonFunctionDefs) -> Self {
        Self::ServiceWithNonFunctionDefs(value)
    }
}

impl std::fmt::Display for ServiceWithNonFunctionDefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
        CompilerOutput {
            title: format!("service \"{}\" should only contain function definitions. Please remove everything else.", self.service_name),
            location: self.location.clone(),
            annotation: "".to_string(),
            suggestion: None,
        }
                .to_string(AnnotationType::Error),
    )
    }
}
