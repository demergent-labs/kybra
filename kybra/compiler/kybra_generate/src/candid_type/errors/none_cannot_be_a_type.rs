use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct NoneCannotBeAType {
    pub location: Location,
}

impl NoneCannotBeAType {
    pub fn err_from_expr(expr_kind: &SourceMapped<&Located<ExprKind>>) -> Error {
        Self {
            location: expr_kind.create_location(),
        }
        .into()
    }
}

impl From<NoneCannotBeAType> for Error {
    fn from(value: NoneCannotBeAType) -> Self {
        Self::NoneCannotBeAType(value)
    }
}

impl Display for NoneCannotBeAType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = "None must not be used as a type, but only as a value. Consider using kybra.null or kybra.void as appropriate".to_string();
        let annotation = "Ambiguous None here".to_string();
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
