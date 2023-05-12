use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct TooManyDecorators {
    pub canister_name: String,
    pub method_name: String,
    pub location: Location,
}

impl TooManyDecorators {
    pub fn err_from_stmt(
        stmt_kind: &SourceMapped<&Located<StmtKind>>,
        canister_name: &str,
        method_name: &str,
    ) -> Error {
        Self {
            canister_name: canister_name.to_string(),
            location: stmt_kind.create_location(),
            method_name: method_name.to_string(),
        }
        .into()
    }
}

impl From<TooManyDecorators> for Error {
    fn from(value: TooManyDecorators) -> Self {
        Self::TooManyDecorators(value)
    }
}

impl Display for TooManyDecorators {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "{}.{} has too many decorators. Please remove all but either @service_update or @service_query",
            self.canister_name, self.method_name
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
