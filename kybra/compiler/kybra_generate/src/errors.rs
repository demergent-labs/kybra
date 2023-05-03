use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};
use std::fmt;

use crate::candid_type::service::errors::{ClassMustHaveMethods, ClassWithNotFunctionDefs};
use crate::source_map::GetSourceInfo;

#[derive(Clone, Debug)]
pub enum Error {
    GuardFunctionNotFound(String),
    TypeNotFound(String),
    Unreachable(String),
    ClassMustHaveMethods(ClassMustHaveMethods),
    ClassWithNotFunctionDefs(ClassWithNotFunctionDefs),
    DictionaryUnpackingOperatorNotSupported(Message),
    FirstParamMustBeSelf(Message),
    FirstParamMustNotBeSelf(Message),
    FuncFormatting(Message),
    GuardFunctionName(Message),
    GuardFunctionParam(Message),
    GuardFunctionReturn(Message),
    InlineFuncNotSupported(Message),
    InvalidAnnAssign(Message),
    InvalidAssign(Message),
    InvalidClass(Message),
    InvalidDecorator(Message),
    InvalidMember(Message),
    InvalidSubscriptable(Message),
    InvalidTarget(Message),
    IteratorUnpackingOperatorNotSupported(Message),
    NoneCantBeAType(Message),
    MissingDecorator(Message),
    MultipleTargets(Message),
    MultipleHeartBeat(Message),
    MultipleInit(Message),
    MultipleInspectMessage(Message),
    MultiplePostUpgrade(Message),
    MultiplePreUpgrade(Message),
    MustBeSubscript(Message),
    NotAnArray(Message),
    NotAnOpt(Message),
    NotAPrimitive(Message),
    NotATuple(Message),
    NotATypeRef(Message),
    ParamTypeAnnotationRequired(Message),
    ReturnTypeAnnotationRequired(Message),
    ReturnTypeMode(Message),
    ReturnTypeMustBeVoid(Message),
    TargetMustBeAName(Message),
    TooManyDecorators(Message),
    WrongDecorator(Message),
    UnsupportedType(Message),
    T(Message),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compiler_output: &dyn std::fmt::Display = match self {
            Error::GuardFunctionNotFound(error) => error,
            Error::TypeNotFound(error) => error,
            Error::Unreachable(error) => error,
            Error::ClassMustHaveMethods(error) => error,
            Error::ClassWithNotFunctionDefs(error) => error,
            Error::DictionaryUnpackingOperatorNotSupported(error) => error,
            Error::FirstParamMustBeSelf(error) => error,
            Error::FirstParamMustNotBeSelf(error) => error,
            Error::FuncFormatting(error) => error,
            Error::GuardFunctionName(error) => error,
            Error::GuardFunctionParam(error) => error,
            Error::GuardFunctionReturn(error) => error,
            Error::InlineFuncNotSupported(error) => error,
            Error::InvalidAnnAssign(error) => error,
            Error::InvalidAssign(error) => error,
            Error::InvalidClass(error) => error,
            Error::InvalidDecorator(error) => error,
            Error::InvalidMember(error) => error,
            Error::InvalidSubscriptable(error) => error,
            Error::InvalidTarget(error) => error,
            Error::IteratorUnpackingOperatorNotSupported(error) => error,
            Error::NoneCantBeAType(error) => error,
            Error::MissingDecorator(error) => error,
            Error::MultipleTargets(error) => error,
            Error::MultipleHeartBeat(error) => error,
            Error::MultipleInit(error) => error,
            Error::MultipleInspectMessage(error) => error,
            Error::MultiplePostUpgrade(error) => error,
            Error::MultiplePreUpgrade(error) => error,
            Error::MustBeSubscript(error) => error,
            Error::NotAnArray(error) => error,
            Error::NotAnOpt(error) => error,
            Error::NotAPrimitive(error) => error,
            Error::NotATuple(error) => error,
            Error::NotATypeRef(error) => error,
            Error::ParamTypeAnnotationRequired(error) => error,
            Error::ReturnTypeAnnotationRequired(error) => error,
            Error::ReturnTypeMode(error) => error,
            Error::ReturnTypeMustBeVoid(error) => error,
            Error::TargetMustBeAName(error) => error,
            Error::TooManyDecorators(error) => error,
            Error::WrongDecorator(error) => error,
            Error::UnsupportedType(error) => error,
            Error::T(error) => error,
        };

        write!(f, "{}", compiler_output)
    }
}

pub type KybraResult<T> = Result<T, Error>;

pub trait CollectResults<T> {
    fn collect_results(self) -> Result<Vec<T>, Vec<Error>>;
}

impl<I, T> CollectResults<T> for I
where
    I: Iterator<Item = Result<T, Vec<Error>>>,
{
    fn collect_results(self) -> Result<Vec<T>, Vec<Error>> {
        let mut errors = Vec::new();
        let mut ok_values = Vec::new();

        self.for_each(|result| match result {
            Ok(ok_value) => ok_values.push(ok_value),
            Err(errs) => errors.extend(errs),
        });

        if errors.is_empty() {
            Ok(ok_values)
        } else {
            Err(errors)
        }
    }
}

pub fn unreachable() -> Error {
    Error::Unreachable("Oops! Looks like we introduced a bug while refactoring. Please open a ticket at https://github.com/demergent-labs/kybra/issues/new".to_string())
}

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

impl fmt::Display for Message {
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

pub trait CreateLocation {
    fn create_location(&self) -> Location;
}

impl<T> CreateLocation for T
where
    T: GetSourceInfo,
{
    fn create_location(&self) -> Location {
        return Location {
            origin: self.get_origin(),
            line_number: self.get_line_number(),
            source: self.get_source(),
            range: self.get_range(),
        };
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

impl From<cdk_framework::act::abstract_canister_tree::Error> for crate::Error {
    fn from(value: cdk_framework::act::abstract_canister_tree::Error) -> Self {
        match value {
            cdk_framework::act::abstract_canister_tree::Error::TypeNotFound(type_ref_name) => {
                crate::Error::TypeNotFound(format!(
                    "The type {} is used, but never defined.",
                    type_ref_name
                ))
            }
            cdk_framework::act::abstract_canister_tree::Error::GuardFunctionNotFound(
                guard_function_name,
            ) => crate::Error::GuardFunctionNotFound(format!(
                "The guard function {} is used, but never defined.",
                guard_function_name
            )),
        }
    }
}
