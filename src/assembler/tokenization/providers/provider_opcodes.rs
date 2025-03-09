use crate::assembler::tokenization::providers::provider::{ProviderResponse, TokenProvider};
use crate::assembler::tokenization::raw_token::RawTokenVariant;
use std::fmt::Debug;

#[derive(Debug)]
pub struct OpcodeProvider {
    input: String,
}

impl TokenProvider for OpcodeProvider {
    fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    fn give(&mut self, ch: char) -> ProviderResponse {
        if ch.is_whitespace() {
            ProviderResponse::TokenFinished(RawTokenVariant::Opcode, self.input.clone())
        } else if ch.is_alphabetic() {
            self.input.push(ch);
            ProviderResponse::Accepted
        } else {
            ProviderResponse::Destroyed
        }
    }

    fn request_end(&mut self) -> Option<(RawTokenVariant, String)> {
        Some((RawTokenVariant::Opcode, self.input.clone()))
    }
}
