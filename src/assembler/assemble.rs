use super::{
    assembly_error::{AssemblyError, AssemblyErrorVariant},
    grammar::token_pattern::AmbiguousToken,
    grammar::assemble::{find_matching_pattern, construct_instruction},
    types::token::{Token, TokenVariant::*},
    tokenization::tokenize::tokenize,
};
use std::collections::HashMap;

pub fn assemble(src: &str) -> Result<Vec<u32>, AssemblyError> {
    let mut instructions = Vec::new();

    let token_lines = tokenize(&src)?;

    let (token_lines, labels) = extract_labels(token_lines);

    for (instruction, tokens) in token_lines.iter().enumerate() {
        let ambiguous_tokens = make_tokens_ambiguous(tokens);

        let pattern = match find_matching_pattern(&ambiguous_tokens) {
            None => return Err(AssemblyError { line: tokens[0].line, variant: AssemblyErrorVariant::UnknownTokenPattern }),
            Some(pattern) => pattern,
        };

        let constructed_instruction = construct_instruction(&tokens, &pattern.bit_pattern, &pattern.encoding, &labels, instruction)?;
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

fn make_tokens_ambiguous(tokens: &Vec<Token>) -> Vec<AmbiguousToken> {
    let mut ambiguous_tokens = Vec::with_capacity(tokens.len());
    tokens.iter().for_each(|token| ambiguous_tokens.push(AmbiguousToken::from(&token.variant)));
    ambiguous_tokens
}
