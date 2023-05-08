use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct IteratorUnpackingOperatorNotSupported {
    pub location: Location,
}

impl IteratorUnpackingOperatorNotSupported {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<IteratorUnpackingOperatorNotSupported> for Error {
    fn from(value: IteratorUnpackingOperatorNotSupported) -> Self {
        Self::IteratorUnpackingOperatorNotSupported(value)
    }
}

impl Display for IteratorUnpackingOperatorNotSupported {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = "the iterator unpacking operator (*) is not supported".to_string();
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
