use rustpython_parser::ast::{ArgData, Located, StmtKind};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

impl SourceMapped<&Located<StmtKind>> {
    pub fn return_type_annotation_required_error(&self) -> Error {
        Error::ReturnTypeAnnotationRequired(self.create_error_message(
            "Return type annotation required",
            "",
            None,
        ))
    }

    pub fn dictionary_unpacking_operator_not_supported_error(&self) -> Error {
        let title = "the dictionary unpacking operator (**) is not supported";
        Error::DictionaryUnpackingOperatorNotSupported(self.create_error_message(title, "", None))
    }

    pub fn iterator_unpacking_operator_not_supported_error(&self) -> Error {
        let title = "the iterator unpacking operator (*) is not supported";
        Error::IteratorUnpackingOperatorNotSupported(self.create_error_message(title, "", None))
    }

    pub fn first_parameter_must_be_self_error(&self) -> Error {
        let title = "method must be an instance method. Add \"self\" as the first parameter";
        Error::FirstParamMustBeSelf(self.create_error_message(title, "", None))
    }

    pub fn first_parameter_must_not_be_self_error(&self) -> Error {
        let title = "first parameter must not be \"self\"";
        Error::FirstParamMustNotBeSelf(self.create_error_message(title, "", None))
    }
}

impl SourceMapped<&Located<ArgData>> {
    pub fn param_type_annotation_required_error(&self) -> Error {
        let param_name = self.node.arg.clone();
        let title = format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", param_name);
        Error::ParamTypeAnnotationRequired(self.create_error_message(title.as_str(), "", None))
    }
}
