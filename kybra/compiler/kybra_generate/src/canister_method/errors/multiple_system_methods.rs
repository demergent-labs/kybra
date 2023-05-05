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
pub struct MultipleSystemMethods {
    pub method_type: CanisterMethodType,
    pub locations: Vec<Location>,
}

impl MultipleSystemMethods {
    pub fn err_from_stmt(
        stmt_kinds: &Vec<SourceMapped<&Located<StmtKind>>>,
        method_type: CanisterMethodType,
    ) -> Error {
        Self {
            locations: stmt_kinds
                .iter()
                .map(|stmt| stmt.create_location())
                .collect(),
            method_type,
        }
        .into()
    }
}

impl From<MultipleSystemMethods> for Error {
    fn from(value: MultipleSystemMethods) -> Self {
        Self::MultipleSystemMethods(value)
    }
}

impl Display for MultipleSystemMethods {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!(
            "Multiple {} methods where defined, only one is allowed.",
            self.method_type
        );
        let annotation = "".to_string();
        let suggestion = None;

        // TODO this can't use compiler output since it has multiple locations
        write!(
            f,
            "{}",
            CompilerOutput {
                title,
                location: self.locations[0].clone(),
                annotation,
                suggestion,
            }
            .to_string(AnnotationType::Error),
        )
    }
}
