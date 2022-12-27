pub mod token_length;

use regex::Regex;
use rustpython_parser::ast::Location;

use crate::py_ast::what_is_it::WhatIsIt;

#[derive(Clone)]
pub struct SourceMap {
    pub file_contents: String,
    pub lines: Vec<String>,
    pub token_lines: Vec<String>,
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

pub trait Locatable {
    fn get_start_row(&self) -> usize;
    fn get_start_col(&self) -> usize;
    fn get_token_length(&self) -> usize;
}

impl SourceMap {
    pub fn new(source: String, py_file_name: &str) -> SourceMap {
        let lines: Vec<_> = source.split("\n").map(|line| line.to_string()).collect();
        let token_lines = lines
            .iter()
            .map(|line| {
                let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                let s = re.replace_all(&line, "");
                s.to_string()
            })
            .collect();
        SourceMap {
            file_contents: source,
            lines,
            file_name: py_file_name.to_string(),
            token_lines,
        }
    }

    /// Alright start on the first line if our item is on line 1 then we want to make sure that the current line is less than the line number.
    /// If so add the the
    pub fn generate_span<T>(&self, token: &T) -> Span
    where
        T: Locatable,
        T: WhatIsIt,
    {
        let start = self.lines.iter().enumerate().fold(0, |acc, line_enum| {
            let (index, line) = line_enum;

            if index < token.get_start_row() - 1 {
                return acc + line.len();
            }

            if index == token.get_start_row() - 1 {
                return acc + index + token.get_start_col();
            }

            acc
        });
        // The goal here is to take the token length and turn it into a end. The problem is that we can't see special characters or
        // whitespace. So what we are going to do is start at the start index. We are going to iterate over all of the characters
        // in the source and if it's a white space or a special character then we are going to skip it
        // otherwise we are going to decrament our count,
        let re = Regex::new(r"[A-Za-z0-9]").unwrap();
        let mut real_count = 0;
        let mut end = 0;
        let token_length = token.get_token_length();
        // eprintln!("The token length is {}", token_length);
        // TODO okay this is working mostly well... The problem is that any special characters afterwards aren't included. But
        // we can't go to the next non special character, because what if the next one starts with special characters.
        for char_enum in self.file_contents[start - 1..].chars().enumerate() {
            let (index, char) = char_enum;
            // eprintln!("This is the char we are looking at ({}) the index is {} and we are shooting for 70ish I think", char, index);
            if re.is_match(&char.to_string()) {
                real_count += 1;
                // eprintln!(
                //     "The token_length is {}, and so far the real count is {}",
                //     token_length, real_count
                // );
                if real_count == token_length {
                    end = start + index;
                    break;
                }
            }
        }
        // eprintln!("We are making a span for this token:");
        // token.what_is_it();
        // eprintln!("This is the start {} and the end {}", start, end);
        Span { start, end }
    }
}

/// The span here is going to represent the absolute character count
pub struct Span {
    start: usize,
    end: usize,
}

impl SourceMap {
    /// Get the source text of the token.
    pub fn get_text(&self, span: Span) -> String {
        self.file_contents[span.start - 1..span.end].to_string()
    }

    /// Get the start character location and the end character location of a token
    /// TODO apparently the span will be good enough for here.
    /// Incorrect the range is not the range for the token in the whole source map
    /// Rather it is the range within the the source.
    /// So we want to get the column and the row and use that to get the range
    pub fn get_range(&self, span: Span, location: Location) -> (usize, usize) {
        let end = self.lines[self.get_start_row(location) - 1..self.get_end_row(&span) - 1]
            .iter()
            .fold(0, |acc, line| acc + line.len() + 1);
        // eprintln!("When caclulating the end this is the ");
        let end = end + self.get_end_col(&span);
        (self.get_start_col(location), end)
    }

    /// Get the source text surrounding the token.
    pub fn get_source(&self, span: Span, location: Location) -> String {
        let first_line = self.get_start_row(location);
        // eprintln!(
        //     "The first line is {} and the text at that line is {}",
        //     first_line,
        //     self.lines[first_line - 1]
        // );
        let last_line = self.get_end_row(&span);
        // eprintln!(
        //     "The last line is {} and the text at that line is {}",
        //     last_line,
        //     self.lines[last_line - 1]
        // );
        let result = if first_line == last_line {
            // eprintln!("We decided that the first line and the last line were the same");
            self.lines[first_line - 1].clone()
        } else {
            // eprintln!("We decided that the first line and the last line were not the same");
            // eprintln!("{:#?}", &self.lines[first_line - 1..last_line]);
            self.lines[first_line - 1..last_line]
                .iter()
                .fold(String::new(), |acc, line| format!("{}{}\n", acc, line))
        };
        result[..result.len() - 1].to_string()
    }

    /// get_source and replace token in span with replacement
    pub fn generate_modified_source(
        &self,
        span: &Span,
        location: Location,
        replacement: &String,
    ) -> String {
        format!(
            "{}{}{}",
            self.get_well_formed_line(location),
            replacement,
            self.get_well_formed_end_line(span)
        )
    }

    /// Get the span that would cover the replacement it it was the actual code
    pub fn generate_modified_range(&self, span: Span, replacement: &String) -> (usize, usize) {
        (span.start, span.start + replacement.len())
    }

    /// Get the file name of that the span is in
    pub fn get_origin(&self) -> String {
        self.file_name.clone()
    }

    /// Get the line number at the beginning of the span
    pub fn get_line_number(&self, location: Location) -> usize {
        location.row()
    }
}

impl SourceMap {
    /// Get the column of the beginning
    fn get_start_col(&self, location: Location) -> usize {
        location.column() - 1 // TODO verify: I think that RustPython columns start at 1 but that the columns we are expecting here start at 0
    }

    /// Get the column of the end
    fn get_end_col(&self, span: &Span) -> usize {
        // let loc = self.lookup_char_pos(location);
        // let line_length = self.get_source(span, length).len();
        // // TODO this is the hackiest way to do this... I hope there is a better way
        // (loc.col + length).min(line_length);

        // eprintln!("Col End span: ({}, {})", span.start, span.end);
        let mut char_count = 0;
        for line in &self.lines {
            // eprintln!(
            //     "\"{}\" is {} characters long.\n\tChar count: {}. Span end: {}\n\tResult: {}",
            //     line,
            //     line.len() + 1,
            //     char_count,
            //     span.end,
            //     char_count + line.len() + 1
            // );
            if char_count + line.len() + 1 > span.end {
                // If adding the whole length of the next line plus 1 for the
                // new line we split on is bigger than the end span marker then
                // we know the end span marker is in this line somewhere.
                let total = char_count + line.len() + 1;
                // eprintln!("Final Total: {}", total);
                // eprintln!("Calculating end col:");
                // eprintln!("line.len() + 1 - (total - span.end) + 1 = result");
                // eprintln!(
                //     "{} + 1 - ({} - {}) = {}",
                //     line.len(),
                //     total,
                //     span.end,
                //     line.len() + 1 - (total - span.end) + 1
                // );
                return line.len() - (total - span.end) + 1; // Plus one for the new line that was deleted, plus one because we want columns not index // TODO I actually think we don't want the one from the new line because we aren't looking at the new line here?
            }
            char_count += line.len() + 1 //Add the length of the line plus the new line character that is no longer there
        }
        panic!("Unexpected end of file")
    }

    fn get_start_row(&self, location: Location) -> usize {
        location.row()
    }

    fn get_end_row(&self, span: &Span) -> usize {
        // eprintln!(
        //     "We are attempting to get the row that has character {} on it",
        //     span.end
        // );
        let mut char_count = 0;
        for line_enum in self.lines.iter().enumerate() {
            let (index, line) = line_enum;
            // eprintln!(
            //     "The line ({} long) is {}.\nThe char count so far is {}. The span end is {}",
            //     line.len() + 1,
            //     line,
            //     char_count,
            //     span.end
            // );
            if char_count + line.len() + 1 > span.end {
                return index + 1; // Add one to the index because the index starts at 0 and the rows start at 1
            }
            char_count += line.len() + 1
        }
        panic!("Unexpected end of file")
    }

    /// Get the beginning of the first line where the token happens.
    fn get_well_formed_line(&self, location: Location) -> String {
        let line = &self.lines[self.get_start_row(location) - 1];
        line[..self.get_start_col(location)].to_string()
    }

    /// Get the last part of the line where it ends
    /// TODO It should get the last line which might not be the first line
    /// TODO the discription should describe the todo above
    fn get_well_formed_end_line(&self, span: &Span) -> String {
        let line = &self.lines[self.get_end_row(span) - 1];
        line[..self.get_end_col(span)].to_string()
    }
}
