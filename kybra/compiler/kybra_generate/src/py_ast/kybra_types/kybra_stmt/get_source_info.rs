use rustpython_parser::ast::{Located, StmtKind};

use super::KybraStmt;
use crate::source_map::{token_length::TokenLength, GetSourceInfo, Locatable};

impl Locatable for Located<StmtKind> {
    fn get_start_row(&self) -> usize {
        self.location.row()
    }

    fn get_start_col(&self) -> usize {
        self.location.column()
    }

    fn get_token_length(&self) -> usize {
        TokenLength::get_token_length(self)
    }

    fn get_location(&self) -> rustpython_parser::ast::Location {
        self.location
    }
}

impl GetSourceInfo for KybraStmt<'_> {
    fn get_text(&self) -> String {
        self.source_map
            .get_text(self.source_map.generate_span(self.stmt_kind))
    }

    fn get_range(&self) -> (usize, usize) {
        self.source_map.get_range(
            self.source_map.generate_span(self.stmt_kind),
            self.stmt_kind.location,
        )
    }

    fn get_source(&self) -> String {
        self.source_map.get_source(
            self.source_map.generate_span(self.stmt_kind),
            self.stmt_kind.location,
        )
    }

    fn generate_modified_source(&self, replacement: &String) -> String {
        self.source_map.generate_modified_source(
            &self.source_map.generate_span(self.stmt_kind),
            self.stmt_kind.location,
            replacement,
        )
    }

    fn generate_modified_range(&self, replacement: &String) -> (usize, usize) {
        self.source_map
            .generate_modified_range(self.source_map.generate_span(self.stmt_kind), replacement)
    }

    fn get_origin(&self) -> String {
        self.source_map.get_origin()
    }

    fn get_line_number(&self) -> usize {
        self.source_map.get_line_number(self.stmt_kind.location)
    }
}
