use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct FirstParamMustNotBeSelf {
    pub location: Location,
}

impl FirstParamMustNotBeSelf {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<FirstParamMustNotBeSelf> for Error {
    fn from(value: FirstParamMustNotBeSelf) -> Self {
        Self::FirstParamMustNotBeSelf(value)
    }
}

impl Display for FirstParamMustNotBeSelf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = "first parameter must not be \"self\"".to_string();
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
