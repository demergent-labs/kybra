use rustpython_parser::ast::{ArgData, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn return_type_annotation_required_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Return type annotation required", "", None)]
    }

    pub fn dictionary_unpacking_operator_not_supported_error(&self) -> Vec<Message> {
        let title = "the dictionary unpacking operator (**) is not supported";
        vec![self.create_error_message(title, "", None)]
    }

    pub fn iterator_unpacking_operator_not_supported_error(&self) -> Vec<Message> {
        let title = "the iterator unpacking operator (*) is not supported";
        vec![self.create_error_message(title, "", None)]
    }

    pub fn first_parameter_must_be_self_error(&self) -> Vec<Message> {
        let title = "method must be an instance method. Add \"self\" as the first parameter";
        vec![self.create_error_message(title, "", None)]
    }

    pub fn first_parameter_must_not_be_self_error(&self) -> Vec<Message> {
        let title = "first parameter must not be \"self\"";
        vec![self.create_error_message(title, "", None)]
    }
}

impl SourceMapped<&Located<ArgData>> {
    pub fn param_type_annotation_required_error(&self) -> Vec<Message> {
        let param_name = self.node.arg.clone();
        let title = format!("parameter \"{}\" is missing a type annotation. All method parameters must specify their type.", param_name);
        vec![self.create_error_message(title.as_str(), "", None)]
    }
}
