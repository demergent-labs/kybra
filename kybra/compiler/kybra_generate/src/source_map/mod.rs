pub mod token_length;

use rustpython_parser::ast::Location;

#[derive(Clone)]
pub struct SourceMap {
    pub lines: Vec<String>,
    pub file_name: String,
}

#[derive(Clone)]
pub struct Source {}

pub trait GetSourceInfo {
    fn get_text(&self) -> String;
    fn get_range(&self) -> (usize, usize);
    fn get_source(&self) -> String;
    fn generate_modified_source(&self, replacement: &String) -> String;
    fn generate_modified_range(&self, replacement: &String) -> (usize, usize);
    fn get_origin(&self) -> String;
    fn get_line_number(&self) -> usize;
}

impl SourceMap {
    pub fn get_text(&self, span: Location, length: usize) -> String {
        let line = self.get_source(span);
        line[self.get_start_col(span)..self.get_end_col(span, length)].to_string()
    }

    pub fn get_range(&self, span: Location, length: usize) -> (usize, usize) {
        let start = self.get_start_col(span);
        let end = self.get_end_col(span, length);
        (start, end)
    }

    pub fn get_source(&self, span: Location) -> String {
        let line_result = self.lookup_line(span);
        match line_result {
            Ok(source_file_and_line) => {
                let line_number = source_file_and_line.line;
                match source_file_and_line.sf.get_line(line_number) {
                    Some(line) => line.to_string(),
                    None => "Unable to find line in source code".to_string(),
                }
            }
            Err(_) => "Unable to find line in source code".to_string(),
        }
    }

    pub fn generate_modified_source(
        &self,
        span: Location,
        length: usize,
        replacement: &String,
    ) -> String {
        format!(
            "{}{}{}",
            self.get_well_formed_line(span),
            replacement,
            self.get_well_formed_end_line(span, length)
        )
    }

    pub fn generate_modified_range(&self, span: Location, replacement: &String) -> (usize, usize) {
        (
            self.get_start_col(span),
            self.get_start_col(span) + replacement.len(),
        )
    }

    pub fn get_origin(&self, span: Location) -> String {
        let loc = self.get_loc(span);
        loc.file.name.to_string()
    }

    pub fn get_line_number(&self, span: Location) -> usize {
        let loc = self.get_loc(span);
        loc.line
    }
}

impl SourceMap {
    fn lookup_char_pos(&self, location: Location) -> Loc {
        Loc {
            file: SourceFile {
                file: self.lines.clone(),
                name: self.file_name.clone(),
            },
            line: location.row(),
            col: location.column(),
        }
    }

    fn lookup_line(&self, location: Location) -> Result<SourceFileAndLine, String> {
        Ok(SourceFileAndLine {
            sf: SourceFile {
                file: self.lines.clone(),
                name: self.file_name.clone(),
            },
            line: location.row(),
        })
    }

    fn get_loc(&self, span: Location) -> Loc {
        self.lookup_char_pos(span)
    }

    fn get_start_col(&self, span: Location) -> usize {
        let loc = self.lookup_char_pos(span);
        loc.col
    }

    fn get_end_col(&self, span: Location, length: usize) -> usize {
        let loc = self.lookup_char_pos(span);
        let line_length = self.get_source(span).len();
        // TODO this is the hackiest way to do this... I hope there is a better way
        (loc.col + length).min(line_length)
    }

    fn get_well_formed_line(&self, span: Location) -> String {
        let line = self.get_source(span);
        line[..self.get_start_col(span)].to_string()
    }

    fn get_well_formed_end_line(&self, span: Location, length: usize) -> String {
        let line = self.get_source(span);
        line[self.get_end_col(span, length)..].to_string()
    }
}

pub struct Loc {
    pub file: SourceFile,
    pub line: usize,
    pub col: usize,
}

pub struct SourceFile {
    pub file: Vec<String>,
    pub name: String,
}

impl SourceFile {
    fn get_line(self, line_number: usize) -> Option<String> {
        self.file.get(line_number - 1).cloned()
    }
}

struct SourceFileAndLine {
    pub sf: SourceFile,
    pub line: usize,
}

impl Loc {}
