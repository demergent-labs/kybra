use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn stable_b_tree_map_node_format_error(&self) -> Vec<Message> {
        vec![self.create_error_message(
            "This is not how a stable b tree map node ought to be formatted",
            "",
            None,
        )]
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn invalid_memory_id_error(&self) -> Vec<Message> {
        vec![self.create_error_message("invalid memory id", "", None)]
    }

    pub fn missing_memory_id_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Missing memory Id", "", None)]
    }

    pub fn memory_id_must_by_integer_constant_error(&self) -> Vec<Message> {
        vec![self.create_error_message("memory id must be an integer", "", None)]
    }

    pub fn max_size_must_be_integer_constant_error(&self) -> Vec<Message> {
        vec![self.create_error_message("max size must be an integer", "", None)]
    }

    pub fn not_a_stable_b_tree_map_node_error(&self) -> Vec<Message> {
        vec![self.create_error_message("not a stable b tree map node", "", None)]
    }

    pub fn max_size_too_big_error(&self) -> Vec<Message> {
        // Max size must be less than MAX_U32 + 1
        vec![self.create_error_message("max size must be less than MAX_U32 + 1", "", None)]
    }

    pub fn memory_id_too_big_error(&self) -> Vec<Message> {
        // Max size must be less than MAX_U32 + 1
        vec![self.create_error_message("Memory ID must be less than MAX_U32 + 1", "", None)]
    }

    pub fn memory_id_must_be_an_integer_error(&self) -> Vec<Message> {
        vec![self.create_error_message("memory id must be an integer", "", None)]
    }

    // pub fn generics_must_be_expressed_as_a_tuple_error(&self) -> Vec<Message> {
    //     vec![self.create_error_message("generics must be expressed as a tuple", "", None)]
    // }

    pub fn max_key_size_missing_error(&self) -> Vec<Message> {
        vec![self.create_error_message("max_key_size_missing", "", None)]
    }

    pub fn max_value_size_missing_error(&self) -> Vec<Message> {
        vec![self.create_error_message("max_value_size missing", "", None)]
    }

    pub fn memory_id_must_be_non_negative(&self) -> Vec<Message> {
        vec![self.create_error_message("memory id must be non negative", "", None)]
    }

    pub fn max_size_must_be_non_negative(&self) -> Vec<Message> {
        vec![self.create_error_message("max size must be non negative", "", None)]
    }
}
