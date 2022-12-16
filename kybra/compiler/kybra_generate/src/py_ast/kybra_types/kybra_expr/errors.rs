use crate::errors::{CreateMessage, ErrorMessage, Suggestion};

use super::KybraExpr;

impl KybraExpr<'_> {
    pub fn invalid_subscript_value_error(&self) -> ErrorMessage {
        let title =
            "Only Async, list, manual, opt, or tuple are allowed subscripts for candid values";
        let annotation = "Invalid subscript here";
        self.create_error_message(title, annotation, None)
    }

    pub fn not_array_error(&self) -> ErrorMessage {
        let suggestion = Suggestion {
            title: "This error should only show up for Kybra developers that used a method wrong. If you see this error, please create an issue for us.".to_string(),
            source: None,
            range: None,
            annotation: None,
            import_suggestion: None,
        };
        self.create_error_message(
            "This is not an array. Only arrays should reach this point.",
            "",
            Some(suggestion),
        )
    }

    pub fn not_tuple_error(&self) -> String {
        "This is is not a tuple".to_string()
    }

    pub fn not_opt_error(&self) -> String {
        "This is is not an opt".to_string()
    }
}
