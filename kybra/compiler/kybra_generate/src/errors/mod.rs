pub mod collect_results;
pub mod compiler_output;
pub mod message;
pub mod unreachable;

use std::fmt;

use crate::{
    candid_type::{
        errors::{
            InvalidMember, InvalidName, InvalidSubscriptable, NoneCannotBeAType,
            NotExactlyOneTarget, UnsupportedType,
        },
        func::errors::{
            FuncCallTakesOneArg, FuncFormatting, InlineFuncNotSupported, ReturnTypeMode,
        },
        service::errors::{
            InvalidDecorator, MissingDecorator, ServiceMustHaveMethods, ServiceWithNotFunctionDefs,
            TooManyDecorators, WrongDecorator,
        },
    },
    canister_method::errors::{GuardFunctionName, MultipleSystemMethods, ReturnTypeMustBeVoid},
    guard_function::errors::{GuardFunctionParam, GuardFunctionReturn},
    method_utils::errors::{
        DictionaryUnpackingOperatorNotSupported, FirstParamMustBeSelf, FirstParamMustNotBeSelf,
        IteratorUnpackingOperatorNotSupported, ParamTypeAnnotationRequired,
        ReturnTypeAnnotationRequired,
    },
    stable_b_tree_map_nodes::errors::{
        InvalidMemoryId, MaxKeySizeMissing, MaxSizeMustBeInteger, MaxSizeMustBeNonNegative,
        MaxSizeTooBig, MaxValueSizeMissing, MemoryIdMustBeAnInteger, MemoryIdMustBeInteger,
        MemoryIdMustBeNonNegative, MemoryIdTooBig, MissingMemoryId, StableBTreeMapNodeFormat,
    },
};

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
    ClassMustHaveMethods(ServiceMustHaveMethods),
    ClassWithNotFunctionDefs(ServiceWithNotFunctionDefs),
    DictionaryUnpackingOperatorNotSupported(DictionaryUnpackingOperatorNotSupported),
    FirstParamMustBeSelf(FirstParamMustBeSelf),
    FirstParamMustNotBeSelf(FirstParamMustNotBeSelf),
    FuncFormatting(FuncFormatting),
    FuncCallTakesOneArg(FuncCallTakesOneArg),
    GuardFunctionName(GuardFunctionName),
    GuardFunctionParam(GuardFunctionParam),
    GuardFunctionReturn(GuardFunctionReturn),
    InlineFuncNotSupported(InlineFuncNotSupported),
    InvalidDecorator(InvalidDecorator),
    InvalidMember(InvalidMember),
    InvalidName(InvalidName),
    InvalidSubscriptable(InvalidSubscriptable),
    IteratorUnpackingOperatorNotSupported(IteratorUnpackingOperatorNotSupported),
    NoneCannotBeAType(NoneCannotBeAType),
    MissingDecorator(MissingDecorator),
    MultipleSystemMethods(MultipleSystemMethods),
    NotExactlyOneTarget(NotExactlyOneTarget),
    ParamTypeAnnotationRequired(ParamTypeAnnotationRequired),
    ReturnTypeAnnotationRequired(ReturnTypeAnnotationRequired),
    ReturnTypeMode(ReturnTypeMode),
    ReturnTypeMustBeVoid(ReturnTypeMustBeVoid),
    TooManyDecorators(TooManyDecorators),
    WrongDecorator(WrongDecorator),
    UnsupportedType(UnsupportedType),
    SbtmInvalidMemoryId(InvalidMemoryId),
    SbtmMaxKeySizeMissing(MaxKeySizeMissing),
    SbtmMaxSizeMustBeInteger(MaxSizeMustBeInteger),
    SbtmMaxSizeMustBeNonNegative(MaxSizeMustBeNonNegative),
    SbtmMaxSizeTooBig(MaxSizeTooBig),
    SbtmMaxValueSizeMissing(MaxValueSizeMissing),
    SbtmMemoryIdMustBeAnInteger(MemoryIdMustBeAnInteger),
    SbtmMemoryIdMustBeInteger(MemoryIdMustBeInteger),
    SbtmMemoryIdMustBeNonNegative(MemoryIdMustBeNonNegative),
    SbtmMemoryIdTooBig(MemoryIdTooBig),
    SbtmMissingMemoryId(MissingMemoryId),
    SbtmStableBTreeMapNodeFormat(StableBTreeMapNodeFormat),
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
            Error::FuncCallTakesOneArg(error) => error,
            Error::GuardFunctionName(error) => error,
            Error::GuardFunctionParam(error) => error,
            Error::GuardFunctionReturn(error) => error,
            Error::InlineFuncNotSupported(error) => error,
            Error::InvalidDecorator(error) => error,
            Error::InvalidMember(error) => error,
            Error::InvalidName(error) => error,
            Error::InvalidSubscriptable(error) => error,
            Error::IteratorUnpackingOperatorNotSupported(error) => error,
            Error::NoneCannotBeAType(error) => error,
            Error::MissingDecorator(error) => error,
            Error::NotExactlyOneTarget(error) => error,
            Error::MultipleSystemMethods(error) => error,
            Error::ParamTypeAnnotationRequired(error) => error,
            Error::ReturnTypeAnnotationRequired(error) => error,
            Error::ReturnTypeMode(error) => error,
            Error::ReturnTypeMustBeVoid(error) => error,
            Error::TooManyDecorators(error) => error,
            Error::WrongDecorator(error) => error,
            Error::UnsupportedType(error) => error,
            Error::SbtmInvalidMemoryId(error) => error,
            Error::SbtmMaxKeySizeMissing(error) => error,
            Error::SbtmMaxSizeMustBeInteger(error) => error,
            Error::SbtmMaxSizeMustBeNonNegative(error) => error,
            Error::SbtmMaxSizeTooBig(error) => error,
            Error::SbtmMaxValueSizeMissing(error) => error,
            Error::SbtmMemoryIdMustBeAnInteger(error) => error,
            Error::SbtmMemoryIdMustBeInteger(error) => error,
            Error::SbtmMemoryIdMustBeNonNegative(error) => error,
            Error::SbtmMemoryIdTooBig(error) => error,
            Error::SbtmMissingMemoryId(error) => error,
            Error::SbtmStableBTreeMapNodeFormat(error) => error,
        };

        write!(f, "{}", compiler_output)
    }
}

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
