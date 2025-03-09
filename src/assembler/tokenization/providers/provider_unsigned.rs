use crate::assembler::tokenization::providers::provider::{ProviderResponse, TokenProvider};
use crate::assembler::tokenization::raw_token::RawTokenVariant;

#[derive(Debug)]
pub struct UnsignedProvider {
    input: String,
}

impl TokenProvider for UnsignedProvider {
    fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    fn give(&mut self, ch: char) -> ProviderResponse {
        if self.input.is_empty() && !ch.is_numeric() {
            return ProviderResponse::Destroyed;
        }

        if ch.is_numeric() {
            self.input.push(ch);
            ProviderResponse::Accepted
        } else if !self.input.is_empty() {
            // If we've gathered at least one digit and encounter a non-digit,
            // finish the token
            ProviderResponse::TokenFinished(RawTokenVariant::Unsigned, self.input.parse().unwrap())
        } else {
            ProviderResponse::Destroyed
        }
    }

    fn request_end(&mut self) -> Option<(RawTokenVariant, String)> {
        if self.input.is_empty() {
            None
        } else {
            Some((RawTokenVariant::Unsigned, self.input.parse().unwrap()))
        }
    }
}
