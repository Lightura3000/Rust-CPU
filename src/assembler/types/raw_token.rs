use std::str::FromStr;
use crate::assembler::types::{
    token::{Token, TokenVariant},
    register::Register,
    opcode::Opcode,
    token_error::{TokenizationError, TokenizationErrorVariant}
};

#[derive(Debug, PartialEq, Clone)]
pub enum RawTokenVariant {
    Text,
    Unsigned,
    Signed,
    Label,
    Register,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RawToken {
    pub variant: RawTokenVariant,
    pub value: String,
    pub line: usize,
}

impl TryFrom<RawToken> for Token {
    type Error = TokenizationError;

    fn try_from(token: RawToken) -> Result<Self, Self::Error> {
        let variant = match token.variant {
            RawTokenVariant::Text => {
                match Opcode::from_str(&token.value) {
                    Ok(opc) => TokenVariant::Opcode(opc),
                    Err(_) => return Err(TokenizationError {
                        line: token.line,
                        position: None,
                        variant: TokenizationErrorVariant::OpcodeNotRecognised,
                    }),
                }
            }
            RawTokenVariant::Unsigned => {
                match token.value.parse::<u16>() {
                    Ok(unsigned) => TokenVariant::Unsigned(unsigned),
                    Err(error) => return Err(TokenizationError {
                        line: token.line,
                        position: None,
                        variant: TokenizationErrorVariant::ParseIntError(error),
                    })
                }
            }
            RawTokenVariant::Signed => {
                match token.value.parse::<i16>() {
                    Ok(signed) => TokenVariant::Signed(signed),
                    Err(error) => return Err(TokenizationError {
                        line: token.line,
                        position: None,
                        variant: TokenizationErrorVariant::ParseIntError(error),
                    }),
                }
            }
            RawTokenVariant::Label => TokenVariant::Label(token.value.to_owned()),
            RawTokenVariant::Register => {
                match Register::from_str(&token.value) {
                    Ok(reg) => TokenVariant::Register(reg),
                    Err(error) => return Err(TokenizationError {
                        line: token.line,
                        position: None,
                        variant: TokenizationErrorVariant::StringError(error),
                    }),
                }
            }
        };

        Ok(Token {
            line: token.line,
            variant,
        })
    }
}
