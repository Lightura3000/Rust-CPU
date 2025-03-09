use std::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct TokenizationError {
    pub line: usize,
    pub position: usize,
    pub variant: TokenizationErrorVariant,
}

#[derive(Debug)]
pub enum TokenizationErrorVariant {
    InvalidCharacter(char),
    OpcodeNotRecognised,
    ParseIntError(ParseIntError),
    ParseRegisterError,
}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.variant {
            TokenizationErrorVariant::InvalidCharacter(char) => format!("Invalid character '{}'", char),
            TokenizationErrorVariant::OpcodeNotRecognised => "Unrecognized opcode".to_string(),
            TokenizationErrorVariant::ParseIntError(_) => "Integer parsing error".to_string(),
            TokenizationErrorVariant::ParseRegisterError => "Unparsable register".to_string(),
        };
        write!(f, "{}", str)
    }
}
