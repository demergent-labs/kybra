use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct InvalidName {
    pub location: Location,
}

impl InvalidName {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<InvalidName> for Error {
    fn from(value: InvalidName) -> Self {
        Self::InvalidName(value)
    }
}

impl Display for InvalidName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "A single valid name for the type must be provided. For example `my_bool` in `my_bool = Alias[bool]`"
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
