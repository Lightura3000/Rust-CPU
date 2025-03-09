use crate::assembler::{
    assembly_error::{AssemblyError, AssemblyErrorVariant},
    grammar::construct_instruction::{construct_instruction, find_matching_pattern},
    grammar::token_pattern::AmbiguousToken,
    tokenization::tokenize::Tokenizer,
    tokenization::raw_token::RawToken,
    tokenization::token::{Token, TokenVariant::*},
    tokenization::tokenization_error::TokenizationError,
};
use std::collections::HashMap;

pub fn assemble(src: String) -> Result<Vec<u32>, AssemblyError> {
    let mut instructions = Vec::new();

    let mut tokenizer = Tokenizer::new(src);
    let raw_tokens = tokenizer.tokenize()?;
    let token_stream = make_raw_tokens_normal(raw_tokens)?;
    let token_lines = collect_into_lines(token_stream);

    let (token_lines, labels) = extract_labels(token_lines);

    for (instruction, tokens) in token_lines.iter().enumerate() {
        let ambiguous_tokens = make_tokens_ambiguous(tokens);

        let pattern = match find_matching_pattern(&ambiguous_tokens) {
            None => return Err(AssemblyError {
                line: tokens[0].line,
                column: None,
                variant: AssemblyErrorVariant::UnknownTokenPattern
            }),
            Some(pattern) => pattern,
        };

        let constructed_instruction = construct_instruction(tokens, &pattern.bit_pattern, &pattern.encoding, &labels, instruction)?;
        instructions.push(constructed_instruction);
    }

    Ok(instructions)
}

fn extract_labels(token_lines: Vec<Vec<Token>>) -> (Vec<Vec<Token>>, HashMap<String, usize>) {
    let mut line_tokens = Vec::new();
    let mut labels = HashMap::new();

    let mut instruction = 0;

    for tokens in token_lines {
        if let Label(name) = &tokens[0].variant {
            labels.insert(name.to_owned(), instruction);
        } else {
            line_tokens.push(tokens);
            instruction += 1;
        }
    }

    (line_tokens, labels)
}

fn make_tokens_ambiguous(tokens: &[Token]) -> Vec<AmbiguousToken> {
    let mut ambiguous_tokens = Vec::with_capacity(tokens.len());
    tokens.iter().for_each(|token| ambiguous_tokens.push(AmbiguousToken::from(&token.variant)));
    ambiguous_tokens
}

fn collect_into_lines(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut lines: Vec<Vec<Token>> = vec![];
    let mut current_line = None;

    for token in tokens {
        if let Some(line) = current_line {
            if line == token.line {
                if let Some(last_line) = lines.last_mut() {
                    last_line.push(token);
                }
            } else {
                current_line = Some(token.line);
                lines.push(vec![token]);
            }
        } else {
            current_line = Some(token.line);
            lines.push(vec![token]);
        }
    }

    lines
}

fn make_raw_tokens_normal(raw_tokens: Vec<RawToken>) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::with_capacity(raw_tokens.len());

    for raw_token in raw_tokens {
        match Token::try_from(raw_token) {
            Ok(token) => tokens.push(token),
            Err(err) => {
                println!("{:#?}", err);
                return Err(err);
            }
        }
    }

    Ok(tokens)
}
