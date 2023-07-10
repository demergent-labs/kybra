use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct FuncCallTakesOneArg {
    pub param_count: usize,
    pub location: Location,
}

impl FuncCallTakesOneArg {
    pub fn err_from_expr(
        expr_kind: &SourceMapped<&Located<ExprKind>>,
        param_count: usize,
    ) -> Error {
        Self {
            location: expr_kind.create_location(),
            param_count,
        }
        .into()
    }
}

impl From<FuncCallTakesOneArg> for Error {
    fn from(value: FuncCallTakesOneArg) -> Self {
        Self::FuncCallTakesOneArg(value)
    }
}

impl Display for FuncCallTakesOneArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "Func() takes only one parameter. {} were given",
            self.param_count
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
