use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct InvalidClass {
    pub location: Location,
}

impl InvalidClass {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<InvalidClass> for Error {
    fn from(value: InvalidClass) -> Self {
        Self::InvalidClass(value)
    }
}

impl Display for InvalidClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = "For a class to be included in your canister definition it must be either a Record or a Variant.".to_string();
        let annotation = "illegal class here".to_string();
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
