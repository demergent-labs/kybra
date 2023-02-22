use rustpython_parser::ast::{Located, StmtKind};

use crate::{
    errors::{Contents, Message},
    source_map::{GetSourceInfo, SourceMapped},
};

impl SourceMapped<&Located<StmtKind>> {
    pub(super) fn record_default_value_warning(&self) -> Message {
        Message::Warning(Contents {
            title: "WARNING: Default values are not supported and will be ignored".to_string(),
            origin: self.get_origin(),
            line_number: self.get_line_number(),
            source: self.get_source(),
            range: self.get_range(),
            annotation: "Default value used here".to_string(),
            suggestion: None,
        })
    }
}
