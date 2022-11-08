use crate::errors::{ErrorMessage, Message};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn invalid_subscript_value_error(&self) -> ErrorMessage {
        ErrorMessage {
            message: Message {
                title: "Only Async, list, manual, opt, or tuple are allowed subscripts for candid values"
                    .to_string(),
                origin: self.source_map.get_origin(self.located_expr.location),
                line_number: self.source_map.get_line_number(self.located_expr.location),
                source: self.source_map.get_source(self.located_expr.location),
                range: self.source_map.get_range(self.located_expr.location),
                annotation: "Invalid subscript here".to_string(),
                suggestion: None,
            },
        }
    }

    pub(super) fn not_array_error(&self) -> String {
        "This is is not an array".to_string()
    }

    pub(super) fn not_tuple_error(&self) -> String {
        "This is is not a tuple".to_string()
    }

    pub(super) fn not_opt_error(&self) -> String {
        "This is is not an opt".to_string()
    }
}
