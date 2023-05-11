use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct MaxSizeTooBig {
    pub location: Location,
}

impl MaxSizeTooBig {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<MaxSizeTooBig> for Error {
    fn from(value: MaxSizeTooBig) -> Self {
        Self::SbtmMaxSizeTooBig(value)
    }
}

impl Display for MaxSizeTooBig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("Max size must be less than Max_U32 + 1");
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
