pub mod collect_results;
pub mod compiler_output;
pub mod unreachable;

use std::fmt;

use crate::{
    candid_type::{
        errors::{
            InvalidMember, InvalidName, InvalidSubscriptable, NoneCannotBeAType,
            NotExactlyOneTarget, UnsupportedType,
        },
        func::errors::{FuncCallTakesOneArg, FuncFormatting, ReturnTypeMode},
        service::errors::{
            MissingDecorator, ServiceMustHaveMethods, ServiceWithNonFunctionDefs,
            TooManyDecorators, WrongDecorator,
        },
    },
    canister_method::errors::{GuardFunctionName, MultipleSystemMethods, ReturnTypeMustBeVoid},
    guard_function::errors::{GuardFunctionParam, GuardFunctionReturn},
    method_utils::errors::{
        DictionaryUnpackingOperatorNotSupported, FirstParamMustBeSelf, FirstParamMustNotBeSelf,
        IteratorUnpackingOperatorNotSupported, ParamTypeAnnotationRequired,
        ReturnTypeAnnotationRequired, TooManyParams,
    },
    py_ast,
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
pub use unreachable::Unreachable;

#[derive(Clone, Debug)]
pub enum Error {
    DictionaryUnpackingOperatorNotSupported(DictionaryUnpackingOperatorNotSupported),
    FirstParamMustBeSelf(FirstParamMustBeSelf),
    FirstParamMustNotBeSelf(FirstParamMustNotBeSelf),
    FuncFormatting(FuncFormatting),
    FuncCallTakesOneArg(FuncCallTakesOneArg),
    GuardFunctionName(GuardFunctionName),
    GuardFunctionNotFound(String),
    GuardFunctionParam(GuardFunctionParam),
    GuardFunctionReturn(GuardFunctionReturn),
    InvalidMember(InvalidMember),
    InvalidName(InvalidName),
    InvalidSubscriptable(InvalidSubscriptable),
    IteratorUnpackingOperatorNotSupported(IteratorUnpackingOperatorNotSupported),
    MissingDecorator(MissingDecorator),
    MultipleCanisterMethodDefinitions(String),
    MultipleGuardFunctionDefinitions(String),
    MultipleSystemMethods(MultipleSystemMethods),
    MultipleTypeDefinitions(String),
    NoneCannotBeAType(NoneCannotBeAType),
    NotExactlyOneTarget(NotExactlyOneTarget),
    ParamTypeAnnotationRequired(ParamTypeAnnotationRequired),
    ReturnTypeAnnotationRequired(ReturnTypeAnnotationRequired),
    ReturnTypeMode(ReturnTypeMode),
    ReturnTypeMustBeVoid(ReturnTypeMustBeVoid),
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
    ServiceMustHaveMethods(ServiceMustHaveMethods),
    ServiceWithNonFunctionDefs(ServiceWithNonFunctionDefs),
    TooManyDecorators(TooManyDecorators),
    TooManyParams(TooManyParams),
    TypeNotFound(String),
    WrongDecorator(WrongDecorator),
    Unreachable(Unreachable),
    UnsupportedType(UnsupportedType),
    IoError(String),
    ParseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compiler_output: &dyn std::fmt::Display = match self {
            Error::GuardFunctionNotFound(error) => error,
            Error::TypeNotFound(error) => error,
            Error::Unreachable(error) => error,
            Error::ServiceMustHaveMethods(error) => error,
            Error::ServiceWithNonFunctionDefs(error) => error,
            Error::DictionaryUnpackingOperatorNotSupported(error) => error,
            Error::FirstParamMustBeSelf(error) => error,
            Error::FirstParamMustNotBeSelf(error) => error,
            Error::FuncFormatting(error) => error,
            Error::FuncCallTakesOneArg(error) => error,
            Error::GuardFunctionName(error) => error,
            Error::GuardFunctionParam(error) => error,
            Error::GuardFunctionReturn(error) => error,
            Error::InvalidMember(error) => error,
            Error::InvalidName(error) => error,
            Error::InvalidSubscriptable(error) => error,
            Error::IteratorUnpackingOperatorNotSupported(error) => error,
            Error::NoneCannotBeAType(error) => error,
            Error::MissingDecorator(error) => error,
            Error::NotExactlyOneTarget(error) => error,
            Error::MultipleCanisterMethodDefinitions(error) => error,
            Error::MultipleGuardFunctionDefinitions(error) => error,
            Error::MultipleSystemMethods(error) => error,
            Error::MultipleTypeDefinitions(error) => error,
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
            Error::TooManyParams(error) => error,
            Error::IoError(error) => error,
            Error::ParseError(error) => error,
        };

        write!(f, "{}", compiler_output)
    }
}

impl From<Error> for Vec<Error> {
    fn from(value: Error) -> Self {
        vec![value]
    }
}

impl From<cdk_framework::act::abstract_canister_tree::Error> for Error {
    fn from(value: cdk_framework::act::abstract_canister_tree::Error) -> Self {
        match value {
            cdk_framework::act::abstract_canister_tree::Error::TypeNotFound(type_ref_name) => {
                Error::TypeNotFound(format!(
                    "The type {} is used, but never defined. Make sure all candid types are defined correctly",
                    type_ref_name
                ))
            }
            cdk_framework::act::abstract_canister_tree::Error::GuardFunctionNotFound(
                guard_function_name,
            ) => Error::GuardFunctionNotFound(format!(
                "The guard function {} is used, but never defined. Make sure all guard functions return kybra.GuardResult",
                guard_function_name
            )),
            cdk_framework::act::abstract_canister_tree::Error::MultipleTypeDefinitions(
                type_ref_name,
            ) => Error::MultipleTypeDefinitions(format!(
                "The type {} is defined multiple times",
                type_ref_name
            )),
            cdk_framework::act::abstract_canister_tree::Error::MultipleGuardFunctionDefinitions(
                guard_function_name,
            ) => Error::MultipleGuardFunctionDefinitions(format!(
                "The guard function {} is defined multiple times",
                guard_function_name
            )),
            cdk_framework::act::abstract_canister_tree::Error::MultipleCanisterMethodDefinitions(
                canister_method_name,
            ) => Error::MultipleCanisterMethodDefinitions(format!(
                "The canister method {} is defined multiple times",
                canister_method_name
            )),
        }
    }
}

impl From<py_ast::Error> for Error {
    fn from(value: py_ast::Error) -> Self {
        match value {
            py_ast::Error::IoError(io_error) => Self::IoError(io_error.to_string()),
            py_ast::Error::ParseError(parse_error) => Self::ParseError(parse_error.to_string()),
        }
    }
}
