use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ReturnTypeMode {
    pub location: Location,
}

impl ReturnTypeMode {
    pub fn err_from_expr(expr_kind: &SourceMapped<&Located<ExprKind>>) -> Error {
        Self {
            location: expr_kind.create_location(),
        }
        .into()
    }
}

impl From<ReturnTypeMode> for Error {
    fn from(value: ReturnTypeMode) -> Self {
        Self::ReturnTypeMode(value)
    }
}

impl Display for ReturnTypeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("return type must be oneway, query, or update");
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
