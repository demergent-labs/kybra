use annotate_snippets::snippet::AnnotationType;
use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CompilerOutput, CreateLocation, CreateMessage, Location},
    source_map::SourceMapped,
    Error,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn class_with_not_function_defs_error(&self, canister_name: &str) -> Error {
        Error::ClassWithNotFunctionDefs(ClassWithNotFunctionDefs {
            class_name: canister_name.to_string(),
            location: self.create_location(),
        })
    }

    pub fn class_must_have_methods_error(&self, canister_name: &str) -> Error {
        Error::ClassMustHaveMethods(ClassMustHaveMethods {
            class_name: canister_name.to_string(),
            location: self.create_location(),
        })
    }

    pub fn missing_decorator_error(&self, canister_name: &String, method_name: &String) -> Error {
        let title = format!(
            "{}.{} is missing a @service_query or @service_update decorator. Please add it above the method",
            canister_name, method_name
        );
        Error::MissingDecorator(self.create_error_message(title.as_str(), "", None))
    }

    pub fn too_many_decorators_error(&self, canister_name: &String, method_name: &String) -> Error {
        let title = format!(
            "{}.{} has too many decorators. Please remove all but either @service_update or @service_query",
            canister_name, method_name
        );
        Error::TooManyDecorators(self.create_error_message(title.as_str(), "", None))
    }

    pub fn wrong_decorator_error(
        &self,
        canister_name: &String,
        method_name: &String,
        id: &String,
    ) -> Error {
        let title = format!(
            "{}.{} has the wrong decorator: expected @service_update or @service_query, got \"@{}\"",
            canister_name, method_name, id
        );
        Error::WrongDecorator(self.create_error_message(title.as_str(), "", None))
    }

    pub fn invalid_decorator_error(&self, canister_name: &String, method_name: &String) -> Error {
        let title = format!(
            "{}.{} has an invalid decorator. Change it to either @service_update or @service_query",
            canister_name, method_name
        );
        Error::InvalidDecorator(self.create_error_message(title.as_str(), "", None))
    }
}

#[derive(Clone, Debug)]
pub struct ClassWithNotFunctionDefs {
    pub class_name: String,
    pub location: Location,
}

impl ClassWithNotFunctionDefs {
    pub fn err_from_stmt(stmt_kind: &SourceMapped<&Located<StmtKind>>, class_name: &str) -> Error {
        Error::ClassWithNotFunctionDefs(Self {
            class_name: class_name.to_string(),
            location: stmt_kind.create_location(),
        })
    }
}

impl From<Error> for Vec<Error> {
    fn from(value: Error) -> Self {
        vec![value]
    }
}

impl From<ClassWithNotFunctionDefs> for Error {
    fn from(value: ClassWithNotFunctionDefs) -> Self {
        Self::ClassWithNotFunctionDefs(value)
    }
}

trait ToString {
    fn to_cool_string(&self) -> String;
}

impl std::fmt::Display for ClassWithNotFunctionDefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
        CompilerOutput {
            title: format!("class \"{}\" should only contain function definitions. Please remove everything else.", self.class_name),
            location: self.location.clone(),
            annotation: "".to_string(),
            suggestion: None,
        }
                .to_string(AnnotationType::Error),
    )
    }
}

#[derive(Clone, Debug)]
pub struct ClassMustHaveMethods {
    pub class_name: String,
    pub location: Location,
}

impl std::fmt::Display for ClassMustHaveMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            CompilerOutput {
                title: format!("class \"{}\" doesn't have any methods. External canisters are required to expose at least one method.", self.class_name),
                location: self.location.clone(),
                annotation: "".to_string(),
                suggestion: None,
            }.to_string(AnnotationType::Error),
        )
    }
}
