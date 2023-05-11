use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct StableBTreeMapNodeFormat {
    pub location: Location,
}

impl StableBTreeMapNodeFormat {
    pub fn err_from_expr(stmt_kind: &SourceMapped<&Located<ExprKind>>) -> Error {
        Self {
            location: stmt_kind.create_location(),
        }
        .into()
    }
}

impl From<StableBTreeMapNodeFormat> for Error {
    fn from(value: StableBTreeMapNodeFormat) -> Self {
        Self::SbtmStableBTreeMapNodeFormat(value)
    }
}

impl Display for StableBTreeMapNodeFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = "This is not how a stable b tree map node ought to be formatted".to_string();
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
