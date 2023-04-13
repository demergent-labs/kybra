use rustpython_parser::ast::{Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn class_with_not_function_defs_error(&self, canister_name: &String) -> Error {
        let title = format!(
            "class \"{}\" should only contain function definitions. Please remove everything else.",
            canister_name
        );
        Error::ClassWithNotFunctionDefs(self.create_error_message(title.as_str(), "", None))
    }

    pub fn class_must_have_methods_error(&self, canister_name: &String) -> Error {
        let title = format!("class \"{}\" doesn't have any methods. External canisters are required to expose at least one method.", canister_name);
        Error::ClassMustHaveMethods(self.create_error_message(title.as_str(), "", None))
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
