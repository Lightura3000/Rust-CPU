use std::ops::Range;
use std::str::FromStr;
use crate::assembler::{
    opcode::Opcode,
    register::Register,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub line: usize,
    pub variant: TokenVariant,
    pub range: Range<usize>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenVariant {
    Opcode(Opcode),
    Label(String),
    Unsigned(u16),
    Signed(i16),
    Register(Register),
    Bool(bool),
    Unknown,
}

impl From<&str> for TokenVariant {
    fn from(value: &str) -> Self {
        if let Ok(opcode) = FromStr::from_str(value) {
            Self::Opcode(opcode)
        } else if value.chars().nth(0) == Some('.') {
            Self::Label(value.to_owned())
        } else if let Ok(value) = value.parse::<u16>() {
            Self::Unsigned(value)
        } else if let Ok(value) = value.parse::<i16>() {
            Self::Signed(value)
        } else if let Ok(register) = value.try_into() {
            Self::Register(register)
        } else if let Ok(boolean) = value.parse::<bool>() {
            Self::Bool(boolean)
        } else {
            Self::Unknown
        }
    }
}
