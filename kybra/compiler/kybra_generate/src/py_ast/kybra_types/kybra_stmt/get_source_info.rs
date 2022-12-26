use crate::source_map::{token_length::TokenLength, GetSourceInfo};

use super::KybraStmt;

impl GetSourceInfo for KybraStmt<'_> {
    // fn do_something(&self) -> bool {
    //     let location = self.stmt_kind.location;
    //     let stmt = self.stmt_kind.node;
    //     let custom = self.stmt_kind.custom;

    //     true
    // }

    fn get_text(&self) -> String {
        self.source_map
            .get_text(self.stmt_kind.location, self.stmt_kind.get_token_length())
    }

    fn get_range(&self) -> (usize, usize) {
        self.source_map
            .get_range(self.stmt_kind.location, self.stmt_kind.get_token_length())
    }

    fn get_source(&self) -> String {
        self.source_map
            .get_source(self.stmt_kind.location, self.stmt_kind.get_token_length())
    }

    fn generate_modified_source(&self, replacement: &String) -> String {
        self.source_map.generate_modified_source(
            self.stmt_kind.location,
            self.stmt_kind.get_token_length(),
            replacement,
        )
    }

    fn generate_modified_range(&self, replacement: &String) -> (usize, usize) {
        self.source_map
            .generate_modified_range(self.stmt_kind.location, replacement)
    }

    fn get_origin(&self) -> String {
        self.source_map.get_origin(self.stmt_kind.location)
    }

    fn get_line_number(&self) -> usize {
        self.source_map.get_line_number(self.stmt_kind.location)
    }
}
