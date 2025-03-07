use std::num::ParseIntError;

#[derive(Debug)]
pub struct TokenizationError {
    pub line: usize,
    pub position: Option<usize>,
    pub variant: TokenizationErrorVariant,
}

#[derive(Debug)]
pub enum TokenizationErrorVariant {
    InvalidCharacter(char),
    OpcodeNotRecognised,
    ParseIntError(ParseIntError),
    StringError(String),
}
