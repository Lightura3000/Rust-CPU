use crate::assembler::tokenization::providers::provider::{TokenProvider, ProviderResponse};
use crate::assembler::tokenization::providers::provider_labels::LabelProvider;
use crate::assembler::tokenization::providers::provider_opcodes::OpcodeProvider;
use crate::assembler::tokenization::providers::provider_registers::RegisterProvider;
use crate::assembler::tokenization::providers::provider_unsigned::UnsignedProvider;
use crate::assembler::tokenization::raw_token::{RawToken, RawTokenVariant};
use crate::assembler::tokenization::tokenization_error::{TokenizationError, TokenizationErrorVariant};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    pos: usize,
    line: usize,
    column: usize,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: Position,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: Position {
                pos: 0,
                line: 0,
                column: 0,
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<RawToken>, TokenizationError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            if self.position.pos >= self.input.len() {
                break;
            }

            match self.get_next_token() {
                Ok(token) => {
                    println!("Constructed token: {:?}", token);
                    println!();

                    for ch in token.value.chars() {
                        // advance
                        if ch == '\n' {
                            self.position.line += 1;
                            self.position.column = 0;
                        } else {
                            self.position.column += 1;
                        }
                        self.position.pos += 1;
                    }

                    tokens.push(token);
                }
                Err(variant) => return Err(TokenizationError {
                    line: self.position.line,
                    column: self.position.column,
                    variant,
                }),
            }
        }

        Ok(tokens)
    }

    fn get_next_token(&mut self) -> Result<RawToken, TokenizationErrorVariant> {
        // Skip until we have a character
        self.skip_whitespace();
        if self.position.pos >= self.input.len() {
            panic!("Reached end of input while parsing input");
        }

        // Establish all providers we want to use
        let providers: Vec<Box<dyn TokenProvider>> = vec![
            Box::from(OpcodeProvider::new()),
            Box::from(UnsignedProvider::new()),
            Box::from(LabelProvider::new()),
            Box::from(RegisterProvider::new()),
        ];
        let mut provider_results = Vec::with_capacity(1);

        // Try each provider
        for provider in providers {
            if let Some(result) = self.simulate_provider(provider, self.position) {
                provider_results.push(result);
            }
        }

        // Check if there is exactly one provider that gave back a token
        if provider_results.is_empty() {
            Err(TokenizationErrorVariant::NoProviderFinished)
        } else if provider_results.len() == 1 {
            let (variant, value) = provider_results.pop().unwrap();
            Ok(RawToken {
                variant,
                value,
                line: self.position.line,
                column: self.position.column,
            })
        } else {
            Err(TokenizationErrorVariant::MultipleProvidersFinished)
        }
    }

    fn simulate_provider(&mut self, mut provider: Box<dyn TokenProvider>, position: Position) -> Option<(RawTokenVariant, String)> {
        let mut position = position;

        loop {
            if position.pos >= self.input.len() {
                return provider.request_end();
            }

            let ch = self.input[position.pos];

            print!("Gave '{}' to {:?} ---- ", ch, provider);
            let response = provider.give(ch);

            match response {
                ProviderResponse::Accepted => {
                    println!("Provider accepted");
                }
                ProviderResponse::TokenFinished(variant, value) => {
                    println!("variant: '{:?}' value: '{}'", variant, value);
                    return Some((variant, value))
                }
                ProviderResponse::Destroyed => {
                    println!("Provider destroyed");
                    return None
                }
            }

            // advance
            if let Some('\n') = self.input.get(position.pos) {
                position.line += 1;
                position.column = 0;
            } else {
                position.column += 1;
            }

            position.pos += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position.pos < self.input.len() && self.input[self.position.pos].is_whitespace() {
            if self.input[self.position.pos] == '\n' {
                self.position.line += 1;
                self.position.column = 0;
            } else {
                self.position.column += 1;
            }
            self.position.pos += 1;
        }
    }
}
