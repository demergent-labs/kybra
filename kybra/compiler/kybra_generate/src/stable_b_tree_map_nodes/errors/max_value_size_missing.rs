use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct MaxValueSizeMissing {
    pub location: Location,
}

impl MaxValueSizeMissing {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<MaxValueSizeMissing> for Error {
    fn from(value: MaxValueSizeMissing) -> Self {
        Self::SbtmMaxValueSizeMissing(value)
    }
}

impl Display for MaxValueSizeMissing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("Max value size missing");
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
