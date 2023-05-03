use std::fmt::{self, Display, Formatter};

use annotate_snippets::snippet::AnnotationType;

use crate::source_map::GetSourceInfo;

use super::{CompilerOutput, Location, Suggestion};

#[derive(Clone, Debug)]
pub enum Message {
    Error(CompilerOutput),
    Warning(CompilerOutput),
}

impl Message {
    fn to_string(&self) -> String {
        match self {
            Message::Error(compiler_output) => compiler_output.to_string(AnnotationType::Error),
            Message::Warning(compiler_output) => compiler_output.to_string(AnnotationType::Warning),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub trait CreateMessage {
    fn create_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> CompilerOutput;

    fn create_error_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> Message {
        Message::Error(self.create_message(title, annotation, suggestion))
    }

    fn create_warning_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> Message {
        Message::Warning(self.create_message(title, annotation, suggestion))
    }
}

impl<T> CreateMessage for T
where
    T: GetSourceInfo,
{
    fn create_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> CompilerOutput {
        return CompilerOutput {
            title: title.to_string(),
            location: Location {
                origin: self.get_origin(),
                line_number: self.get_line_number(),
                source: self.get_source(),
                range: self.get_range(),
            },
            annotation: annotation.to_string(),
            suggestion,
        };
    }
}
