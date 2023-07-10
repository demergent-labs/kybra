use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use cdk_framework::act::node::canister_method::CanisterMethodType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ReturnTypeMustBeVoid {
    pub method_type: CanisterMethodType,
    pub location: Location,
}

impl ReturnTypeMustBeVoid {
    pub fn err_from_stmt(
        stmt_kind: &SourceMapped<&Located<StmtKind>>,
        method_type: CanisterMethodType,
    ) -> Error {
        Self {
            location: stmt_kind.create_location(),
            method_type,
        }
        .into()
    }
}

impl From<ReturnTypeMustBeVoid> for Error {
    fn from(value: ReturnTypeMustBeVoid) -> Self {
        Self::ReturnTypeMustBeVoid(value)
    }
}

impl Display for ReturnTypeMustBeVoid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "{} method must have an explicit void return type annotation",
            self.method_type
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
