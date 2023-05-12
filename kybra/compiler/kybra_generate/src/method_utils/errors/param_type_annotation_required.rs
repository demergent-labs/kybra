use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{ArgData, Located};

use crate::{
    errors::{CompilerOutput, CreateLocation, Location},
    source_map::SourceMapped,
    Error,
};

#[derive(Clone, Debug)]
pub struct ParamTypeAnnotationRequired {
    pub param_name: String,
    pub location: Location,
}

impl ParamTypeAnnotationRequired {
    pub fn err_from_arg_data(arg_data: &SourceMapped<&Located<ArgData>>) -> Error {
        Self {
            location: arg_data.create_location(),
            param_name: arg_data.node.arg.clone(),
        }
        .into()
    }
}

impl From<ParamTypeAnnotationRequired> for Error {
    fn from(value: ParamTypeAnnotationRequired) -> Self {
        Self::ParamTypeAnnotationRequired(value)
    }
}

impl Display for ParamTypeAnnotationRequired {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let title = format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", self.param_name);
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
