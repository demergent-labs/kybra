use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{CreateMessage, Message},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<StmtKind>> {
    pub fn invalid_memory_id_error(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn missing_memory_id_error(&self) -> Vec<Message> {
        vec![self.create_error_message("Missing memory Id", "", None)]
    }

    pub fn memory_id_error_must_by_integer_constant(&self) -> Vec<Message> {
        vec![self.create_error_message("Missing memory Id", "", None)]
    }

    pub fn max_size_must_be_integer_constant(&self) -> Vec<Message> {
        vec![self.create_error_message("Missing memory Id", "", None)]
    }

    pub fn not_a_stable_b_tree_map_node_error(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn max_size_too_big_error(&self) -> Vec<Message> {
        // Max size must be less than MAX_U32 + 1
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn memory_id_too_big_error(&self) -> Vec<Message> {
        // Max size must be less than MAX_U32 + 1
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn memory_id_must_be_an_integer_error(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }

    // pub fn generics_must_be_expressed_as_a_tuple_error(&self) -> Vec<Message> {
    //     todo!("self.create_error_message(\"\", \"\", None)")
    // }

    pub fn max_key_size_missing_error(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn max_value_size_missing_error(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn memory_id_must_be_non_negative(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }

    pub fn max_size_must_be_non_negative(&self) -> Vec<Message> {
        vec![self.create_error_message("todo", "", None)]
    }
}
