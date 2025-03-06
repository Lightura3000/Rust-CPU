use super::{
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

impl TryFrom<RawToken> for Token {
    type Error = ();

    fn try_from(token: RawToken) -> Result<Self, Self::Error> {
        let variant = match token.variant {
            RawTokenVariant::Text => {
                match Opcode::from_str(&token.value) {
                    Ok(opc) => TokenVariant::Opcode(opc),
                    Err(_) => return Err(()),
                }
            }
            RawTokenVariant::Unsigned => {
                match token.value.parse::<u16>() {
                    Ok(unsigned) => TokenVariant::Unsigned(unsigned),
                    Err(_) => return Err(()),
                }
            }
            RawTokenVariant::Signed => {
                match token.value.parse::<i16>() {
                    Ok(signed) => TokenVariant::Signed(signed),
                    Err(_) => return Err(()),
                }
            }
            RawTokenVariant::Label => TokenVariant::Label(token.value.to_owned()),
            RawTokenVariant::Register => {
                match Register::from_str(&token.value) {
                    Ok(reg) => TokenVariant::Register(reg),
                    Err(_) => return Err(()),
                }
            }
        };

        Ok(Token {
            line: token.line,
            variant,
        })
    }
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
        } else if let Ok(register) = value.try_into() {
            Ok(Self::Register(register))
        } else if let Ok(boolean) = value.parse::<bool>() {
            Ok(Self::Bool(boolean))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RawTokenVariant {
    Text,
    Unsigned,
    Signed,
    Label,
    Register,
}

#[derive(Debug, PartialEq)]
pub struct RawToken {
    pub variant: RawTokenVariant,
    pub value: String,
    pub line: usize,
}
