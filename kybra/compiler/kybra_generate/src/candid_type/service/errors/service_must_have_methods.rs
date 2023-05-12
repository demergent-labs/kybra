use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ServiceMustHaveMethods {
    pub class_name: String,
    pub location: Location,
}

impl ServiceMustHaveMethods {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>, class_name: &str) -> Error {
        Self {
            class_name: class_name.to_string(),
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<ServiceMustHaveMethods> for Error {
    fn from(value: ServiceMustHaveMethods) -> Self {
        Self::ServiceMustHaveMethods(value)
    }
}

impl Display for ServiceMustHaveMethods {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            CompilerOutput {
                title: format!("service \"{}\" doesn't have any methods. Services are required to expose at least one method.", self.class_name),
                location: self.location.clone(),
                annotation: "".to_string(),
                suggestion: None,
            }.to_string(AnnotationType::Error),
        )
    }
}
