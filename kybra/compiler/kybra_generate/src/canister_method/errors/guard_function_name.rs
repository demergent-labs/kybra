use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct GuardFunctionName {
    pub location: Location,
}

impl GuardFunctionName {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<GuardFunctionName> for Error {
    fn from(value: GuardFunctionName) -> Self {
        Self::GuardFunctionName(value)
    }
}

impl Display for GuardFunctionName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("Guard function must be a reference to a function",);
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
