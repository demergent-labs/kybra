use cdk_framework::{act::node::data_type::Array, ToDataType};
use rustpython_parser::ast::{ExprKind, Located};

use crate::{
    errors::{CreateMessage, Message, Suggestion},
    source_map::SourceMapped,
};

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_array(&self) -> bool {
        match &self.node {
            ExprKind::Subscript { value, .. } => match &value.node {
                ExprKind::Name { id, .. } => id == "list",
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_array(&self) -> Option<Array> {
        self.to_array().ok()
    }

    pub fn to_array(&self) -> Result<Array, Message> {
        if !self.is_array() {
            return Err(self.not_array_error());
        }
        match &self.node {
            ExprKind::Subscript { value, slice, .. } => {
                match &value.node {
                    ExprKind::Name { id, .. } => {
                        if id != "list" {
                            return Err(self.not_array_error());
                        }
                    }
                    _ => return Err(self.not_array_error()),
                }
                let kybra_expr = SourceMapped::new(slice.as_ref(), self.source_map.clone());
                Ok(Array {
                    enclosed_type: Box::from(kybra_expr.to_data_type()),
                })
            }
            _ => Err(self.not_array_error()),
        }
    }

    pub fn not_array_error(&self) -> Message {
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
}
