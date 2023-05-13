pub mod token_length;

use std::ops::Deref;

use regex::Regex;
use rustpython_parser::ast::{Located, Location};

use crate::debug::WhatIsIt;

use self::token_length::{TokenLength, REGULAR_CHARACTERS, SPECIAL_CHARACTERS};

#[derive(Clone, Debug)]
pub struct SourceMap {
    pub file_contents: String,
    pub lines: Vec<String>,
    pub token_lines: Vec<String>,
    pub file_name: String,
}

#[derive(Clone, Debug)]
pub struct SourceMapped<T> {
    inner: T,
    pub source_map: SourceMap,
}

impl<T> SourceMapped<T> {
    pub fn new(inner: T, source_map: SourceMap) -> SourceMapped<T> {
        SourceMapped { inner, source_map }
    }
}

impl<T> Deref for SourceMapped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
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

impl<T> GetSourceInfo for SourceMapped<&T>
where
    T: Locatable,
    T: WhatIsIt,
{
    fn get_text(&self) -> String {
        self.source_map
            .get_text(self.source_map.generate_span(self.inner))
    }

    fn get_range(&self) -> (usize, usize) {
        self.source_map.get_range(
            self.source_map.generate_span(self.inner),
            self.get_location(),
        )
    }

    fn get_source(&self) -> String {
        self.source_map.get_source(
            self.source_map.generate_span(self.inner),
            self.get_location(),
        )
    }

    fn generate_modified_source(&self, replacement: &String) -> String {
        self.source_map.generate_modified_source(
            &self.source_map.generate_span(self.inner),
            self.get_location(),
            replacement,
        )
    }

    fn generate_modified_range(&self, replacement: &String) -> (usize, usize) {
        self.source_map
            .generate_modified_range(self.source_map.generate_span(self.inner), replacement)
    }

    fn get_origin(&self) -> String {
        self.source_map.get_origin()
    }

    fn get_line_number(&self) -> usize {
        self.source_map.get_line_number(self.get_location())
    }
}

pub trait Locatable {
    fn get_start_row(&self) -> usize;
    fn get_start_col(&self) -> usize;
    fn get_token_length(&self) -> usize;
    fn get_location(&self) -> Location;
}

impl<T> Locatable for Located<T>
where
    T: TokenLength,
{
    fn get_start_row(&self) -> usize {
        self.location.row()
    }

    fn get_start_col(&self) -> usize {
        self.location.column()
    }

    fn get_token_length(&self) -> usize {
        TokenLength::get_token_length(self)
    }

    fn get_location(&self) -> Location {
        self.location
    }
}

impl SourceMap {
    pub fn new(source: String, py_file_name: &str) -> SourceMap {
        let lines: Vec<_> = source.split("\n").map(|line| line.to_string()).collect();
        let token_lines = lines
            .iter()
            .map(|line| {
                let re = Regex::new(SPECIAL_CHARACTERS).unwrap();
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
        let reg_char_re = Regex::new(REGULAR_CHARACTERS).unwrap();
        let comment_re = Regex::new("#").unwrap();
        let newline_re = Regex::new("\n").unwrap();
        let whitespace_re = Regex::new(r"\s").unwrap();
        let at_re = Regex::new(r"@").unwrap();
        let opening_re = Regex::new(r"[\{\[\(]").unwrap();
        let closing_re = Regex::new(r"[\}\]\)]").unwrap();
        let double_quote_re = Regex::new("\"").unwrap();
        let single_quote_re = Regex::new("'").unwrap();
        let escape_re = Regex::new(r"\\").unwrap();
        let mut real_count = 0;
        let mut end = 0;
        let token_length = token.get_token_length();
        let mut is_comment = false;
        let mut open_count = 0;
        let mut is_double_quote_string = false;
        let mut is_single_quote_string = false;
        let mut is_escaping = false;
        let mut found_escape_character = false;
        let mut found_first_character = false;
        let mut is_decorator = false;
        // eprintln!("The token length is {}", token_length);
        // TODO We don't handle the closing ' or "
        // TODO We don't handle calls that have no parameters. the '()' won't count towards the range
        for char_enum in self.file_contents[start - 1..].chars().enumerate() {
            let (index, char) = char_enum;
            let char = &char.to_string();
            if !found_first_character {
                // This is the first not whitespace character. Do all out necessary processing
                if !whitespace_re.is_match(char) {
                    found_first_character = true;
                    if at_re.is_match(char) {
                        is_decorator = true;
                    }
                }
            }
            if found_escape_character {
                found_escape_character = false;
                is_escaping = true;
            }
            // eprintln!("This is the char we are looking at ({}) the index is {} and we are shooting for 70ish I think", char, index);
            if (is_comment || is_decorator) && !newline_re.is_match(char) {
                // If we are in a comment and the current character doesn't end the comment (with a new line) then we should continue
                continue;
            }
            if newline_re.is_match(char) {
                // If we're in a comment and we hit a new line, end the comment
                is_comment = false;
                is_decorator = false;
                found_first_character = false;
            }
            if (!is_double_quote_string || !is_single_quote_string) && opening_re.is_match(char) {
                // If we find a open brace that isn't in a string or a comment then add to our open count
                // TODO test to see if we need to take into account any string formatting `f'Hello {}'` would that last o be the end? Or since we're going to the end of the quote will we be fine?
                open_count += 1
            }
            if (!is_double_quote_string || !is_single_quote_string) && closing_re.is_match(char) {
                // If we find a close brace that isn't in a string or a comment then subtract from our open count
                open_count -= 1;
            }
            if is_double_quote_string || is_single_quote_string {
                // If we are in a string then handle escape characters
                // Here is how we are going to handle escape characters. When we find one we are going to mark that we found it.
                // When we are on the very next character we will set at the beginning that we are processing it and we will set found to false
                // After that we will set that to false on the next pass
                // We need two variables so we can handle back to back escape characters.
                if !is_escaping && escape_re.is_match(char) {
                    found_escape_character = true;
                }
            }
            if !is_single_quote_string && double_quote_re.is_match(char) {
                // We can't use a continue like we can with a comment because we can ignore all characters in a comment but we cannot ignore the characters in a string
                // If we aren't in a single quote or a comment and we hit a double quote then we are either going to start a double quote or end a double quote.
                if is_double_quote_string {
                    // If we are in a double quote and we see another double quote we end the quote unless the previous character was the escape character
                    if !is_escaping {
                        is_double_quote_string = false;
                    }
                } else {
                    is_double_quote_string = true
                }
            }
            if !is_double_quote_string && single_quote_re.is_match(&char.to_string()) {
                // We can't use a continue like we can with a comment because we can ignore all characters in a comment but we cannot ignore the characters in a string
                // If we aren't in a double quote or a comment and we hit a single quote then we are either going to start a single quote or end a single quote.
                if is_single_quote_string {
                    // If we are in a single quote and we see another single quote we end the quote unless the previous character was the escape character
                    if !is_escaping {
                        is_single_quote_string = false;
                    }
                } else {
                    is_single_quote_string = true
                }
            }
            if comment_re.is_match(&char.to_string()) {
                is_comment = true
            }
            if reg_char_re.is_match(&char.to_string()) {
                if real_count == token_length {
                    // for whatever reason we didn't find all of the closing braces but now we are running into more code so we should stop here.
                    end = start + index - 1; // Don't count the current character. We want to count closing braces not additional characters
                    break;
                }
                real_count += 1;
                // eprintln!(
                //     "The token_length is {}, and so far the real count is {}",
                //     token_length, real_count
                // );
            }
            if real_count == token_length
                && open_count == 0
                && !is_double_quote_string
                && !is_single_quote_string
            {
                end = start + index;
                break;
            }
            is_escaping = false; // We finished processing the character so we have nothing left to escape.
        }
        // eprintln!("We are making a span for this token:");
        // token.what_is_it();
        // eprintln!("This is the start {} and the end {}", start, end);

        // eprintln!("###########################################################");
        // eprintln!("### Here is the remaining bit of text #####################");
        // eprintln!("###########################################################");
        // eprintln!("{}", &self.file_contents[end..]);

        let open_param_re = Regex::new(r"\(").unwrap();
        let new_line_re = Regex::new(r"\n").unwrap();
        let whitespace_re = Regex::new(r"\s").unwrap();
        let close_param_re = Regex::new(r"\)").unwrap();
        let mut found_open = false;
        for char_enum in self.file_contents[end..].chars().enumerate() {
            let (index, char) = char_enum;
            let char = &char.to_string();
            // eprintln!("We are looking at this char: {}", char);
            if !found_open && (whitespace_re.is_match(char) || open_param_re.is_match(char)) {
                // For the opening parenthesis you can have tabs or spaces before it but no new lines
                if new_line_re.is_match(char) {
                    break;
                }
                if open_param_re.is_match(char) {
                    found_open = true
                }
                continue;
            }
            // For the closing parenthesis you can have whatever white space you want. So we'll keep going until we find something that isn't one of those.
            if found_open && !(close_param_re.is_match(char) || whitespace_re.is_match(char)) {
                break;
            }
            if found_open && close_param_re.is_match(char) {
                end += index + 1; // To make up for starting over at 0
                break; // There should be nothing left to see
            }
        }
        if end < start {
            println!(
                "INTERNAL WARNING: End was less than start. start: {} end: {}",
                start, end
            );
            end = start
        }
        Span { start, end }
    }
}

/// The span here is going to represent the absolute character count
#[derive(Debug)]
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
        let start_row = self.get_start_row(location);
        let end_row = match self.get_end_row(&span) {
            end_row if end_row < start_row => {
                println!(
                    "INTERNAL WARNING: End row was less than start row. start: {} end: {}",
                    start_row, end_row
                );
                start_row
            }
            end_row => end_row,
        };
        // End = start + length of all lines between start row and end row + length of the end_row up to the end column
        let start = self.get_start_col(location);
        let end = match self.lines[start_row - 1..end_row - 1]
            .iter()
            .fold(0, |acc, line| acc + line.len() + 1)
            + self.get_end_col(&span)
        {
            end if end < start => {
                println!(
                    "INTERNAL WARNING: End was less than start. start: {} end: {}",
                    start, end
                );
                start
            }
            end => end,
        };
        (start, end)
    }

    /// Get the source text surrounding the token.
    pub fn get_source(&self, span: Span, location: Location) -> String {
        let first_line = self.get_start_row(location);
        // eprintln!(
        //     "The first line is {} and the text at that line is {}",
        //     first_line,
        //     self.lines[first_line - 1]
        // );
        let last_line = match self.get_end_row(&span) {
            end_row if end_row < first_line => {
                println!(
                    "INTERNAL WARNING: End row was less than start row. start: {} end: {}",
                    first_line, end_row
                );
                first_line
            }
            end_row => end_row,
        };
        // eprintln!(
        //     "The last line is {} and the text at that line is {}",
        //     last_line,
        //     self.lines[last_line - 1]
        // );
        if first_line == last_line {
            // eprintln!("We decided that the first line and the last line were the same");
            // eprintln!(
            //     "And here is the source that we are returning: {}",
            //     self.lines[first_line - 1].clone()
            // );
            self.lines[first_line - 1].clone()
        } else {
            // eprintln!("We decided that the first line and the last line were not the same");
            // eprintln!("{:#?}", &self.lines[first_line - 1..last_line]);
            let result = self.lines[first_line - 1..last_line]
                .iter()
                .fold(String::new(), |acc, line| format!("{}{}\n", acc, line));
            result[..result.len() - 1].to_string()
        }
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
                return 1 + line.len() - (total - span.end); // Plus one for the new line that was deleted, plus one because we want columns not index // TODO I actually think we don't want the one from the new line because we aren't looking at the new line here? // Also lets move that 1 to the beginning. If we have a result that is going to be 0 then that is going to come from -1 + 1 and that's going to break our unsigned numbers
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
