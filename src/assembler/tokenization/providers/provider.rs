use crate::assembler::tokenization::raw_token::RawTokenVariant;
use std::fmt::Debug;

pub trait TokenProvider : Debug {
    fn new() -> Self where Self: Sized;
    fn give(&mut self, ch: char) -> ProviderResponse;
    fn request_end(&mut self) -> Option<(RawTokenVariant, String)>;
}

pub enum ProviderResponse {
    Accepted, // Character was accepted
    TokenFinished(RawTokenVariant, String), // The token is finished
    Destroyed, // The provider can't continue anymore
}
