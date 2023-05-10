use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct InvalidSubscriptable {
    pub location: Location,
}

impl InvalidSubscriptable {
    pub fn err_from_expr(expr_kind: &SourceMapped<&Located<ExprKind>>) -> Error {
        Self {
            location: expr_kind.create_location(),
        }
        .into()
    }
}

impl From<InvalidSubscriptable> for Error {
    fn from(value: InvalidSubscriptable) -> Self {
        Self::InvalidSubscriptable(value)
    }
}

impl Display for InvalidSubscriptable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title =
            "Only Async, Vec, Manual, Opt, or Tuple are allowable subscriptables for candid values"
                .to_string();
        let annotation = "Invalid subscriptable here".to_string();
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
