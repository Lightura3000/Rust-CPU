use crate::assembler::types::token::TokenVariant;

pub trait TokenVariantProvider {
    /// Feed a character into the provider.
    fn give_char(&mut self, c: char) -> TokenProviderResponse;
}

#[derive(Debug)]
pub enum TokenProviderResponse {
    /// The given character has been handled successfully
    CharProcessedSuccessfully,

    /// The given character can't be parsed
    InvalidCharNoResult,

    /// The given character isn't part of this token anymore, finished token returned
    TokenFinished(TokenVariant),
}
