pub mod collect_results;
pub mod compiler_output;
pub mod message;

use std::fmt;

use crate::candid_type::service::errors::{
    ClassMustHaveMethods, ClassWithNotFunctionDefs, InvalidDecorator, MissingDecorator,
    TooManyDecorators, WrongDecorator,
};

pub use collect_results::CollectResults;
pub use compiler_output::CompilerOutput;
pub use compiler_output::CreateLocation;
pub use compiler_output::Location;
pub use compiler_output::Suggestion;
pub use message::CreateMessage;
pub use message::Message;

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
    InvalidDecorator(InvalidDecorator),
    InvalidMember(Message),
    InvalidSubscriptable(Message),
    InvalidTarget(Message),
    IteratorUnpackingOperatorNotSupported(Message),
    NoneCantBeAType(Message),
    MissingDecorator(MissingDecorator),
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
    TooManyDecorators(TooManyDecorators),
    WrongDecorator(WrongDecorator),
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

impl From<Error> for Vec<Error> {
    fn from(value: Error) -> Self {
        vec![value]
    }
}

pub fn unreachable() -> Error {
    Error::Unreachable("Oops! Looks like we introduced a bug while refactoring. Please open a ticket at https://github.com/demergent-labs/kybra/issues/new".to_string())
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
