use crate::source_map::GetSourceInfo;

use super::KybraStmt;

impl GetSourceInfo for KybraStmt<'_> {
    fn get_text(&self) -> String {
        self.source_map.get_text(self.stmt_kind.location)
    }

    fn get_range(&self) -> (usize, usize) {
        self.source_map.get_range(self.stmt_kind.location)
    }

    fn get_source(&self) -> String {
        self.source_map.get_source(self.stmt_kind.location)
    }

    fn generate_modified_source(&self, replacement: &String) -> String {
        self.source_map
            .generate_modified_source(self.stmt_kind.location, replacement)
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
