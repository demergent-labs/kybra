use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct InvalidAssign {
    pub location: Location,
}

impl InvalidAssign {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<InvalidAssign> for Error {
    fn from(value: InvalidAssign) -> Self {
        Self::InvalidAssign(value)
    }
}

impl Display for InvalidAssign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = "For a global assignment to be included in your canister definition it must be be either a Tuple or a Type Alias".to_string();
        let annotation = "illegal assignment here".to_string();
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
