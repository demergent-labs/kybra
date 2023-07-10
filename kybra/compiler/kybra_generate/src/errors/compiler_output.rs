use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

use crate::source_map::GetSourceInfo;

#[derive(Clone, Debug)]
pub struct Suggestion {
    pub title: String,
    pub source: Option<String>,
    pub range: Option<(usize, usize)>,
    pub annotation: Option<String>,
    pub import_suggestion: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CompilerOutput {
    pub title: String,
    pub location: Location,
    pub annotation: String,
    pub suggestion: Option<Suggestion>,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub origin: String,
    pub line_number: usize,
    pub source: String,
    pub range: (usize, usize),
}

pub trait CreateLocation {
    fn create_location(&self) -> Location;
}

impl CompilerOutput {
    pub fn to_string(&self, annotation_type: AnnotationType) -> String {
        let error_snippet = Snippet {
            title: Some(Annotation {
                label: Some(&self.title),
                id: None,
                annotation_type,
            }),
            footer: vec![],
            slices: vec![Slice {
                source: &self.location.source,
                line_start: self.location.line_number,
                origin: Some(&self.location.origin),
                fold: true,
                annotations: vec![SourceAnnotation {
                    label: &self.annotation,
                    annotation_type,
                    range: self.location.range,
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
                    line_start: self.location.line_number,
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

impl<T> CreateLocation for T
where
    T: GetSourceInfo,
{
    fn create_location(&self) -> Location {
        return Location {
            origin: self.get_origin(),
            line_number: self.get_line_number(),
            source: "".to_string(),
            range: (0, 0),
        };
    }
}
