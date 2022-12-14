use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use std::fmt;

use crate::source_map::GetSourceInfo;

pub struct Suggestion {
    pub title: String,
    pub source: Option<String>,
    pub range: Option<(usize, usize)>,
    pub annotation: Option<String>,
    pub import_suggestion: Option<String>,
}

pub struct Message {
    pub title: String,
    pub origin: String,
    pub line_number: usize,
    pub source: String,
    pub range: (usize, usize),
    pub annotation: String,
    pub suggestion: Option<Suggestion>,
}

pub struct WarningMessage {
    pub message: Message,
}

pub struct ErrorMessage {
    pub message: Message,
}

impl ErrorMessage {
    fn to_string(&self) -> String {
        self.message.to_string(AnnotationType::Error)
    }
}

impl WarningMessage {
    fn to_string(&self) -> String {
        self.message.to_string(AnnotationType::Warning)
    }
}

impl Message {
    fn to_string(&self, annotation_type: AnnotationType) -> String {
        let error_snippet = Snippet {
            title: Some(Annotation {
                label: Some(&self.title),
                id: None,
                annotation_type,
            }),
            footer: vec![],
            slices: vec![Slice {
                source: &self.source,
                line_start: self.line_number,
                origin: Some(&self.origin),
                fold: true,
                annotations: vec![SourceAnnotation {
                    label: &self.annotation,
                    annotation_type,
                    range: self.range,
                }],
            }],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        };

        match &self.suggestion {
            None => format!("{}", DisplayList::from(error_snippet)),
            Some(suggestion) => {
                let suggestion_source = suggestion.source.clone().unwrap_or(Default::default());
                let suggestion_slice = Slice {
                    source: suggestion_source.as_str(),
                    line_start: self.line_number,
                    origin: None,
                    fold: false,
                    annotations: vec![SourceAnnotation {
                        label: match &suggestion.annotation {
                            Some(annotation) => &annotation,
                            None => "",
                        },
                        annotation_type: AnnotationType::Help,
                        range: suggestion.range.unwrap_or(Default::default()),
                    }],
                };
                let slices = match &suggestion.import_suggestion {
                    Some(import) => vec![
                        Slice {
                            source: &import,
                            line_start: 1,
                            origin: None,
                            annotations: vec![],
                            fold: false,
                        },
                        suggestion_slice,
                    ],
                    None => vec![suggestion_slice],
                };

                let suggestion_snippet = Snippet {
                    title: Some(Annotation {
                        label: Some(&suggestion.title),
                        id: None,
                        annotation_type: AnnotationType::Help,
                    }),
                    footer: vec![],
                    slices,
                    opt: FormatOptions {
                        color: true,
                        ..Default::default()
                    },
                };

                format!(
                    "{}\n{}",
                    DisplayList::from(error_snippet),
                    DisplayList::from(suggestion_snippet)
                )
            }
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for WarningMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub trait CreateMessage {
    fn create_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> Message;

    fn create_error_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> ErrorMessage {
        ErrorMessage {
            message: self.create_message(title, annotation, suggestion),
        }
    }

    fn create_warning_message(
        &self,
        title: &str,
        annotation: &str,
        suggestion: Option<Suggestion>,
    ) -> WarningMessage {
        WarningMessage {
            message: self.create_message(title, annotation, suggestion),
        }
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
    ) -> Message {
        return Message {
            title: title.to_string(),
            origin: self.get_origin(),
            line_number: self.get_line_number(),
            source: self.get_source(),
            range: self.get_range(),
            annotation: annotation.to_string(),
            suggestion,
        };
    }
}
