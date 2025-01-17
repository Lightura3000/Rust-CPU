use crate::assembler::{
    opcode::Opcode,
    register::Register,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub line: usize,
    pub variant: TokenVariant,
}

impl Token {
    pub fn try_construct(line: usize, value: &str) -> Option<Token> {
        match TokenVariant::try_from(value) {
            None => None,
            Some(token_type) => Some(Self { line, variant: token_type }),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenVariant {
    Opcode(Opcode),
    Label(String),
    Unsigned(u16),
    Signed(i16),
    Register(Register),
    Bool(bool),
}

impl TokenVariant {
    fn try_from(value: &str) -> Option<TokenVariant> {
        if let Ok(opcode) = value.try_into() {
            Some(TokenVariant::Opcode(opcode))
        } else if value.chars().nth(0) == Some('.') {
            Some(TokenVariant::Label(value.to_owned()))
        } else if let Ok(value) = value.parse::<u16>() {
            Some(TokenVariant::Unsigned(value))
        } else if let Ok(value) = value.parse::<i16>() {
            Some(TokenVariant::Signed(value))
        } else if let Ok(register) = value.try_into() {
            Some(TokenVariant::Register(register))
        } else if let Ok(boolean) = value.parse::<bool>() {
            Some(TokenVariant::Bool(boolean))
        } else {
            None
        }
    }
}
