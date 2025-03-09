use crate::assembler::{
    types::opcode::Opcode,
    types::register::Register,
    tokenization::token::{Token, TokenVariant},
    tokenization::tokenization_error::{TokenizationError, TokenizationErrorVariant},
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum RawTokenVariant {
    Opcode,
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
    pub column: usize,
}

impl RawToken {
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl TryFrom<RawToken> for Token {
    type Error = TokenizationError;

    fn try_from(token: RawToken) -> Result<Self, Self::Error> {
        let variant = match token.variant {
            RawTokenVariant::Opcode => {
                match Opcode::from_str(&token.value) {
                    Ok(opc) => TokenVariant::Opcode(opc),
                    Err(_) => return Err(TokenizationError {
                        line: token.line,
                        column: token.column,
                        variant: TokenizationErrorVariant::OpcodeNotRecognised,
                    }),
                }
            }
            RawTokenVariant::Unsigned => {
                match token.value.parse::<u16>() {
                    Ok(unsigned) => TokenVariant::Unsigned(unsigned),
                    Err(error) => return Err(TokenizationError {
                        line: token.line,
                        column: token.column,
                        variant: TokenizationErrorVariant::ParseIntError(error),
                    })
                }
            }
            RawTokenVariant::Signed => {
                match token.value.parse::<i16>() {
                    Ok(signed) => TokenVariant::Signed(signed),
                    Err(error) => return Err(TokenizationError {
                        line: token.line,
                        column: token.column,
                        variant: TokenizationErrorVariant::ParseIntError(error),
                    }),
                }
            }
            RawTokenVariant::Label => TokenVariant::Label(token.value.to_owned()),
            RawTokenVariant::Register => {
                match Register::from_str(&token.value) {
                    Ok(reg) => TokenVariant::Register(reg),
                    Err(_) => return Err(TokenizationError {
                        line: token.line,
                        column: token.column,
                        variant: TokenizationErrorVariant::ParseRegisterError,
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
