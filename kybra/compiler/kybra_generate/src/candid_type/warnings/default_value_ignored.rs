use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
};

#[derive(Clone, Debug)]
pub struct DefaultValueIgnored {
    pub location: Location,
}

impl DefaultValueIgnored {
    pub fn new(stmt_kind: &SourceMapped<&Located<StmtKind>>) -> DefaultValueIgnored {
        Self {
            location: stmt_kind.create_location(),
        }
    }
}

impl Display for DefaultValueIgnored {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            CompilerOutput {
                title: "WARNING: Default values are not supported and will be ignored".to_string(),
                location: self.location.clone(),
                annotation: "Default value used here".to_string(),
                suggestion: None,
            }
            .to_string(AnnotationType::Warning),
        )
    }
}
