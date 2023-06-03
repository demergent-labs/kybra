use rustpython_parser::error::ParseError;
use std::io;

pub enum Error {
    IoError(io::Error),
    ParseError(ParseError),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::IoError(io_error)
    }
}

impl From<ParseError> for Error {
    fn from(parse_error: ParseError) -> Self {
        Self::ParseError(parse_error)
    }
}
