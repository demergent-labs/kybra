use crate::errors::ErrorMessage;

use super::KybraExpr;

impl KybraExpr<'_> {
    pub(super) fn type_not_supported_error(&self) -> ErrorMessage {
        ErrorMessage {
            title: "Invalid Type".to_string(),
            origin: self.source_map.get_origin(self.located_expr.location),
            line_number: self.source_map.get_line_number(self.located_expr.location),
            source: self.source_map.get_source(self.located_expr.location),
            range: self.source_map.get_range(self.located_expr.location),
            annotation: "invalid type here".to_string(),
            suggestion: None,
        }
    }

    pub(super) fn invalid_subscript_value_error(&self) -> ErrorMessage {
        ErrorMessage {
            title: "must be opt or list".to_string(),
            origin: todo!(),
            line_number: todo!(),
            source: todo!(),
            range: todo!(),
            annotation: todo!(),
            suggestion: todo!(),
        }
    }

    pub(super) fn not_array_error(&self) -> String {
        "This is is not an array".to_string()
    }

    pub(super) fn not_opt_error(&self) -> String {
        "This is is not an opt".to_string()
    }
}
