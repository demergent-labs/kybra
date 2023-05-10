use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct MissingDecorator {
    pub service_name: String,
    pub method_name: String,
    pub location: Location,
}

impl MissingDecorator {
    pub fn err_from_stmt(
        stmt_kind: &SourceMapped<&Located<StmtKind>>,
        service_name: &str,
        method_name: &str,
    ) -> Error {
        Self {
            service_name: service_name.to_string(),
            location: stmt_kind.create_location(),
            method_name: method_name.to_string(),
        }
        .into()
    }
}

impl From<MissingDecorator> for Error {
    fn from(value: MissingDecorator) -> Self {
        Self::MissingDecorator(value)
    }
}

impl Display for MissingDecorator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "{}.{} is missing a @service_query or @service_update decorator",
            self.service_name, self.method_name
        );
        let annotation = "".to_string();
        let suggestion = None;

        write!(
            f,
            "{}",
            CompilerOutput {
                title,
                location: self.location.clone(),
                annotation,
                suggestion,
            }
            .to_string(AnnotationType::Error),
        )
    }
}
