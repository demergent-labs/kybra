use crate::source_map::{token_length::TokenLength, GetSourceInfo};

use super::KybraExpr;

impl GetSourceInfo for KybraExpr<'_> {
    fn get_text(&self) -> String {
        self.source_map.get_text(
            self.located_expr.location,
            self.located_expr.get_token_length(),
        )
    }

    fn get_range(&self) -> (usize, usize) {
        self.source_map.get_range(
            self.located_expr.location,
            self.located_expr.get_token_length(),
        )
    }

    fn get_source(&self) -> String {
        self.source_map.get_source(
            self.located_expr.location,
            self.located_expr.get_token_length(),
        )
    }

    fn generate_modified_source(&self, replacement: &String) -> String {
        self.source_map.generate_modified_source(
            self.located_expr.location,
            self.located_expr.get_token_length(),
            replacement,
        )
    }

    fn generate_modified_range(&self, replacement: &String) -> (usize, usize) {
        self.source_map
            .generate_modified_range(self.located_expr.location, replacement)
    }

    fn get_origin(&self) -> String {
        self.source_map.get_origin(self.located_expr.location)
    }

    fn get_line_number(&self) -> usize {
        self.source_map.get_line_number(self.located_expr.location)
    }
}
