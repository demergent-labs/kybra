use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct MemoryIdTooBig {
    pub location: Location,
}

impl MemoryIdTooBig {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<MemoryIdTooBig> for Error {
    fn from(value: MemoryIdTooBig) -> Self {
        Self::SbtmMemoryIdTooBig(value)
    }
}

impl Display for MemoryIdTooBig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("Memory ID must be less than MAX_U32 + 1");
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
