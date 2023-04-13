use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CreateMessage, Suggestion},
    source_map::SourceMapped,
    Error,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn not_array_error(&self) -> Error {
        let suggestion = Suggestion {
            title: "This error should only show up for Kybra developers that used a method wrong. If you see this error, please create an issue for us.".to_string(),
            source: None,
            range: None,
            annotation: None,
            import_suggestion: None,
        };
        Error::NotAnArray(self.create_error_message(
            "This is not an array. Only arrays should reach this point.",
            "",
            Some(suggestion),
        ))
    }
}
