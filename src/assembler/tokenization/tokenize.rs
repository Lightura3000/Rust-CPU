use std::str::FromStr;
use crate::assembler::{
    assembly_error::AssemblyError,
    assembly_error::AssemblyErrorVariant,
};
use crate::assembler::tokenization::raw_token::{RawToken, RawTokenVariant};
use crate::assembler::tokenization::token::{Token, TokenVariant};
use crate::assembler::tokenization::tokenization_error::{TokenizationError, TokenizationErrorVariant};

pub fn tokenize(src: &str) -> Result<Vec<Vec<Token>>, AssemblyError> {
    let mut tokens = Vec::new();

    for (line, content) in src.lines().enumerate() {
        let mut line_tokens = Vec::new();

        for (param_index, split) in content.split_whitespace().enumerate() {
            let variant = match TokenVariant::from_str(split) {
                Ok(variant) => variant,
                Err(_) => return Err(AssemblyError {
                    line,
                    column: None,
                    variant: AssemblyErrorVariant::UnrecognizableParam { param_index },
                }),
            };

            line_tokens.push(Token {
                line,
                variant,
                // range: index..(index + split.len()),
            });
        }

        if !line_tokens.is_empty() {
            tokens.push(line_tokens);
        }
    }

    Ok(tokens)
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
    line_position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
            line: 0,
            line_position: 0,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<RawToken>, TokenizationError> {
        let mut tokens = Vec::new();

        loop {
            let next_token = self.get_next_token()?;
            dbg!(&next_token);
            tokens.push(next_token);

            self.skip_whitespace();
            if self.position >= self.input.len() {
                break;
            }
        }

        Ok(tokens)
    }

    fn advance(&mut self) {
        if self.input[self.position] == '\n' {
            self.line += 1;
            self.line_position = 0;
        }

        self.position += 1;
        self.line_position += 1;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let char = self.input.get(self.position);

            match char {
                Some(c) if c.is_whitespace() => self.advance(),
                _ => break,
            }
        }
    }

    fn get_next_token(&mut self) -> Result<RawToken, TokenizationError> {

        // This part should be done even before calling the function
        // and handle the end of input in some way
        self.skip_whitespace();
        if self.position >= self.input.len() {
            panic!("Reached end unexpectantly")
        }
        //////////////

        let current_char = self.input[self.position];

        match current_char {
            'r' => {
                let start_line_pos = self.line_position;
                self.advance();
                Ok(RawToken {
                    variant: RawTokenVariant::Register,
                    value: "r".to_string() + &self.collect_unsigned()?.value,
                    line: self.line,
                    column: start_line_pos,
                })
            }
            '.' => {
                let start_line_pos = self.line_position;
                self.advance();
                Ok(RawToken {
                    variant: RawTokenVariant::Label,
                    value: ".".to_string() + &self.collect_text()?.value,
                    line: self.line,
                    column: start_line_pos,
                })
            }
            '-' => {
                let start_line_pos = self.line_position;
                self.advance();
                Ok(RawToken {
                    variant: RawTokenVariant::Signed,
                    value: "-".to_string() + &self.collect_unsigned()?.value,
                    line: self.line,
                    column: start_line_pos,
                })
            }
            c if c.is_alphabetic() => Ok(self.collect_text()?),
            c if c.is_numeric() => Ok(self.collect_unsigned()?),
            c => Err(TokenizationError {
                line: self.line,
                position: self.position,
                variant: TokenizationErrorVariant::InvalidCharacter(c),
            }),
        }
    }

    fn collect_text(&mut self) -> Result<RawToken, TokenizationError> {
        let start_pos = self.position;
        let start_line_pos = self.line_position;

        let mut value = String::new();

        loop {
            if self.position >= self.input.len() {
                break;
            }

            let c = self.input[self.position];

            if c.is_alphabetic() {
                value.push(c);
            } else if c.is_whitespace() {
                break;
            } else {
                return Err(TokenizationError {
                    line: self.line,
                    position: self.position,
                    variant: TokenizationErrorVariant::InvalidCharacter(c),
                });
            }

            self.advance();
        };

        Ok(RawToken {
            variant: RawTokenVariant::Text,
            value: self.input[start_pos..self.position].iter().collect(),
            line: self.line,
            column: start_line_pos,
        })
    }

    fn collect_unsigned(&mut self) -> Result<RawToken, TokenizationError> {
        let start_pos = self.position;
        let start_line_pos = self.line_position;

        let mut value = String::new();

        loop {
            if self.position >= self.input.len() {
                break;
            }

            let c = self.input[self.position];

            if c.is_numeric() {
                value.push(c);
            } else if c.is_whitespace() {
                break;
            } else {
                return Err(TokenizationError {
                    line: self.line,
                    position: self.position,
                    variant: TokenizationErrorVariant::InvalidCharacter(c),
                });
            }

            self.position += 1;
        };

        Ok(RawToken {
            variant: RawTokenVariant::Unsigned,
            value: self.input[start_pos..self.position].iter().collect(),
            line: self.line,
            column: start_line_pos,
        })
    }
}
