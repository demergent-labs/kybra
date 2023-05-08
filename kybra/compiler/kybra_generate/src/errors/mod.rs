pub mod collect_results;
pub mod compiler_output;
pub mod message;
pub mod unreachable;

use std::fmt;

use crate::candid_type::errors::{
    InvalidMember, InvalidSubscriptable, InvalidTarget, NoneCannotBeAType, NotExactlyOneTarget,
    TargetMustBeAName, UnsupportedType,
};
use crate::candid_type::func::errors::{FuncFormatting, InlineFuncNotSupported, ReturnTypeMode};
use crate::candid_type::service::errors::{
    ClassMustHaveMethods, ClassWithNotFunctionDefs, InvalidDecorator, MissingDecorator,
    TooManyDecorators, WrongDecorator,
};
use crate::canister_method::errors::{
    GuardFunctionName, MultipleSystemMethods, ReturnTypeMustBeVoid,
};
use crate::guard_function::errors::{GuardFunctionParam, GuardFunctionReturn};

pub use collect_results::CollectResults;
pub use compiler_output::CompilerOutput;
pub use compiler_output::CreateLocation;
pub use compiler_output::Location;
pub use compiler_output::Suggestion;
pub use message::CreateMessage;
pub use message::Message;
pub use unreachable::Unreachable;

#[derive(Clone, Debug)]
pub enum Error {
    GuardFunctionNotFound(String),
    TypeNotFound(String),
    Unreachable(Unreachable),
    ClassMustHaveMethods(ClassMustHaveMethods),
    ClassWithNotFunctionDefs(ClassWithNotFunctionDefs),
    DictionaryUnpackingOperatorNotSupported(Message),
    FirstParamMustBeSelf(Message),
    FirstParamMustNotBeSelf(Message),
    FuncFormatting(FuncFormatting),
    GuardFunctionName(GuardFunctionName),
    GuardFunctionParam(GuardFunctionParam),
    GuardFunctionReturn(GuardFunctionReturn),
    InlineFuncNotSupported(InlineFuncNotSupported),
    InvalidDecorator(InvalidDecorator),
    InvalidMember(InvalidMember),
    InvalidSubscriptable(InvalidSubscriptable),
    InvalidTarget(InvalidTarget),
    IteratorUnpackingOperatorNotSupported(Message),
    NoneCannotBeAType(NoneCannotBeAType),
    MissingDecorator(MissingDecorator),
    MultipleSystemMethods(MultipleSystemMethods),
    NotATuple(Message),
    NotATypeRef(Message),
    NotExactlyOneTarget(NotExactlyOneTarget),
    ParamTypeAnnotationRequired(Message),
    ReturnTypeAnnotationRequired(Message),
    ReturnTypeMode(ReturnTypeMode),
    ReturnTypeMustBeVoid(ReturnTypeMustBeVoid),
    TargetMustBeAName(TargetMustBeAName),
    TooManyDecorators(TooManyDecorators),
    WrongDecorator(WrongDecorator),
    UnsupportedType(UnsupportedType),
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
            Error::InvalidDecorator(error) => error,
            Error::InvalidMember(error) => error,
            Error::InvalidSubscriptable(error) => error,
            Error::InvalidTarget(error) => error,
            Error::IteratorUnpackingOperatorNotSupported(error) => error,
            Error::NoneCannotBeAType(error) => error,
            Error::MissingDecorator(error) => error,
            Error::NotExactlyOneTarget(error) => error,
            Error::MultipleSystemMethods(error) => error,
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
