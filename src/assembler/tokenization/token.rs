use crate::assembler::types::{
    opcode::Opcode,
    register::Register,
};
//use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub line: usize,
    pub variant: TokenVariant,
//    pub range: Range<usize>,
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

impl FromStr for TokenVariant {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(opcode) = FromStr::from_str(value) {
            Ok(Self::Opcode(opcode))
        } else if value.chars().nth(0) == Some('.') {
            Ok(Self::Label(value.to_owned()))
        } else if let Ok(value) = value.parse::<u16>() {
            Ok(Self::Unsigned(value))
        } else if let Ok(value) = value.parse::<i16>() {
            Ok(Self::Signed(value))
        } else if let Ok(register) = Register::from_str(value) {
            Ok(Self::Register(register))
        } else if let Ok(boolean) = value.parse::<bool>() {
            Ok(Self::Bool(boolean))
        } else {
            Err(())
        }
    }
}
