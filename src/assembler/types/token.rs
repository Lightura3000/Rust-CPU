use super::{
    opcode::Opcode,
    register::Register,
};
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub line: usize,
    pub variant: TokenVariant,
    pub range: Range<usize>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenVariant {
    Opcode(Opcode),
    Label(String),
    Unsigned(u16),
    Signed(i16),
    Register(Register),
    Bool(bool),
}

impl TryFrom<&str> for TokenVariant {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(opcode) = FromStr::from_str(value) {
            Ok(Self::Opcode(opcode))
        } else if value.chars().nth(0) == Some('.') {
            Ok(Self::Label(value.to_owned()))
        } else if let Ok(value) = value.parse::<u16>() {
            Ok(Self::Unsigned(value))
        } else if let Ok(value) = value.parse::<i16>() {
            Ok(Self::Signed(value))
        } else if let Ok(register) = value.try_into() {
            Ok(Self::Register(register))
        } else if let Ok(boolean) = value.parse::<bool>() {
            Ok(Self::Bool(boolean))
        } else {
            Err(())
        }
    }
}
