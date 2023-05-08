use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct MaxSizeMustBeNonNegative {
    pub location: Location,
}

impl MaxSizeMustBeNonNegative {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<MaxSizeMustBeNonNegative> for Error {
    fn from(value: MaxSizeMustBeNonNegative) -> Self {
        Self::SbtmMaxSizeMustBeNonNegative(value)
    }
}

impl Display for MaxSizeMustBeNonNegative {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("Max size must be non negative");
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
