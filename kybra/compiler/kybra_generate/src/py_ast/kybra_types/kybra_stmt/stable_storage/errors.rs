use crate::{
    errors::{ErrorMessage, Message},
    py_ast::kybra_types::KybraStmt,
};

impl KybraStmt<'_> {
    pub fn invalid_memory_id_error(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn missing_memory_id_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: Message {
                title: "Missing Memory Id".to_string(),
                origin: "a python file".to_string(),
                line_number: 1,
                source: "This is some code".to_string(),
                range: (0, 0),
                annotation: "".to_string(),
                suggestion: None,
            },
        }
        // todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn not_a_stable_b_tree_map_node_error(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn max_key_size_too_big_error(&self) -> ErrorMessage {
        // Max size must be less than MAX_U32 + 1
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn memory_id_too_big_error(&self) -> ErrorMessage {
        // Max size must be less than MAX_U32 + 1
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn memory_id_must_be_an_integer_error(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn generics_must_be_expressed_as_a_tuple_error(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn max_key_size_missing_error(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn max_value_size_missing_error(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }

    pub fn memory_id_must_be_non_negative(&self) -> ErrorMessage {
        todo!("self.create_error_message(\"\", \"\", None)")
    }
}
