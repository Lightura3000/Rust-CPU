use crate::assembler::tokenization::providers::provider::{ProviderResponse, TokenProvider};
use crate::assembler::tokenization::raw_token::RawTokenVariant;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RegisterProvider {
    input: String,
}

impl TokenProvider for RegisterProvider {
    fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    fn give(&mut self, ch: char) -> ProviderResponse {
        if self.input.is_empty() {
            if ch == 'r' {
                self.input.push(ch);
                ProviderResponse::Accepted
            } else {
                ProviderResponse::Destroyed
            }
        } else if ch.is_whitespace() {
            ProviderResponse::TokenFinished(RawTokenVariant::Register, self.input.clone())
        } else if ch.is_numeric() {
            self.input.push(ch);
            ProviderResponse::Accepted
        } else {
            ProviderResponse::Destroyed
        }
    }

    fn request_end(&mut self) -> Option<(RawTokenVariant, String)> {
        Some((RawTokenVariant::Register, self.input.clone()))
    }
}
