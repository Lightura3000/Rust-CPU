use super::provider::TokenProviderResponse;
use super::provider::TokenVariantProvider;
use crate::assembler::types::token::TokenVariant;

enum NonDecimalBase {
    Binary,
    Hexadecimal,
}

pub struct UIntProvider {
    buffer: String,
    base: Option<NonDecimalBase>,
}

impl UIntProvider {
    pub(crate) fn new() -> Self {
        Self {
            buffer: String::new(),
            base: None,
        }
    }

    fn give_char_none_base(&mut self, c: char) -> TokenProviderResponse {
        match c {
            '0'..='9' => {
                self.buffer.push(c);
                TokenProviderResponse::CharProcessedSuccessfully
            },
            'x' => {
                if self.buffer != "0" {
                    return TokenProviderResponse::InvalidCharNoResult
                }

                self.base = Some(NonDecimalBase::Hexadecimal);
                self.buffer.clear();
                TokenProviderResponse::CharProcessedSuccessfully
            }
            'b' => {
                if self.buffer != "0" {
                    return TokenProviderResponse::InvalidCharNoResult
                }

                self.base = Some(NonDecimalBase::Binary);
                self.buffer.clear();
                TokenProviderResponse::CharProcessedSuccessfully
            }
            _ => match self.buffer.parse() {
                    Ok(unsigned) => TokenProviderResponse::TokenFinished(TokenVariant::Unsigned(unsigned)),
                    Err(_) => TokenProviderResponse::InvalidCharNoResult // TODO: Maybe handle the error more specifially?
                }

        }
    }

    fn give_char_hex_base(&mut self, c: char) -> TokenProviderResponse {
        if c.is_ascii_hexdigit() {
            self.buffer.push(c);
            TokenProviderResponse::CharProcessedSuccessfully
        } else {
            match u16::from_str_radix(&self.buffer, 16) {
                Ok(unsigned) => TokenProviderResponse::TokenFinished(TokenVariant::Unsigned(unsigned)),
                Err(_) => TokenProviderResponse::InvalidCharNoResult,
            }
        }
    }

    fn give_char_binary_base(&mut self, c: char) -> TokenProviderResponse {
        if c == '0' || c == '1' {
            self.buffer.push(c);
            TokenProviderResponse::CharProcessedSuccessfully
        } else {
            match u16::from_str_radix(&self.buffer, 2) {
                Ok(unsigned) => TokenProviderResponse::TokenFinished(TokenVariant::Unsigned(unsigned)),
                Err(_) => TokenProviderResponse::InvalidCharNoResult,
            }
        }
    }
}

impl TokenVariantProvider for UIntProvider {
    fn give_char(&mut self, c: char) -> TokenProviderResponse {
        match self.base {
            None => self.give_char_none_base(c),
            Some(NonDecimalBase::Binary) => self.give_char_binary_base(c),
            Some(NonDecimalBase::Hexadecimal) => self.give_char_hex_base(c),
        }
    }
}
