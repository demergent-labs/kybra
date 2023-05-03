use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct WrongDecorator {
    pub canister_name: String,
    pub method_name: String,
    pub location: Location,
    pub id: String,
}

impl WrongDecorator {
    pub fn err_from_stmt(
        stmt_kind: &SourceMapped<&Located<StmtKind>>,
        canister_name: &str,
        method_name: &str,
        id: &str,
    ) -> Error {
        Self {
            canister_name: canister_name.to_string(),
            location: stmt_kind.create_location(),
            method_name: method_name.to_string(),
            id: id.to_string(),
        }
        .into()
    }
}

impl From<WrongDecorator> for Error {
    fn from(value: WrongDecorator) -> Self {
        Self::WrongDecorator(value)
    }
}

impl Display for WrongDecorator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "{}.{} has the wrong decorator: expected @service_update or @service_query, got \"@{}\"",
            self.canister_name, self.method_name, self.id
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
