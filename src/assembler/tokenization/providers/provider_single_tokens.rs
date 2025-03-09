/*
use crate::assembler::tokenization::providers::provider::{ProviderResponse, TokenProvider};
use crate::assembler::tokenization::raw_token::RawTokenVariant;

#[derive(Debug)]
pub struct SingleTokenProvider;

impl TokenProvider for SingleTokenProvider {
    fn new() -> Self {
        Self
    }

    fn give(&mut self, ch: char) -> ProviderResponse {
        match ch {
            '=' => ProviderResponse::TokenFinished(RawTokenVariant::Equals, ch.to_string()),
            ';' => ProviderResponse::TokenFinished(RawTokenVariant::Semicolon, ch.to_string()),
            _ => ProviderResponse::Destroyed,
        }
    }

    fn request_end(&mut self) -> Option<(RawTokenVariant, String)> {
        None
    }
}
*/