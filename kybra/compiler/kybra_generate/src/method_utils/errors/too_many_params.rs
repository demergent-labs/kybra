use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    method_utils::params::InternalOrExternal,
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct TooManyParams {
    pub internal_or_external: InternalOrExternal,
    pub location: Location,
}

impl TooManyParams {
    pub fn err_from_stmt(
        stmt_kind: &SourceMapped<&Located<StmtKind>>,
        internal_or_external: InternalOrExternal,
    ) -> Error {
        Self {
            location: stmt_kind.create_location(),
            internal_or_external,
        }
        .into()
    }
}

impl From<TooManyParams> for Error {
    fn from(value: TooManyParams) -> Self {
        Self::TooManyParams(value)
    }
}

impl Display for TooManyParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = match self.internal_or_external {
            InternalOrExternal::Internal => {
                format!("Too many params: Kybra only supports 5 parameters for canister method definitions")
            }
            InternalOrExternal::External => {
                format!("Too many params: Kybra only supports 6 parameters for service method definitions including 'self'")
            }
        };
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
