use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct FuncFormatting {
    pub location: Location,
}

impl FuncFormatting {
    pub fn err_from_expr(expr_kind: &SourceMapped<&Located<ExprKind>>) -> Error {
        Self {
            location: expr_kind.create_location(),
        }
        .into()
    }
}

impl From<FuncFormatting> for Error {
    fn from(value: FuncFormatting) -> Self {
        Self::FuncFormatting(value)
    }
}

impl Display for FuncFormatting {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "Funcs should be in the form Func(Mode[[param1, params2, ..., paramN], returns])"
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
