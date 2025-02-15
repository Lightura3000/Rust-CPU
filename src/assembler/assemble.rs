use super::{
    assembly_error::{AssemblyError, AssemblyErrorVariant},
    grammar::grammar,
    tokenize::tokenize,
    types::token::{Token, TokenVariant::*},
};
use std::collections::HashMap;

pub fn assemble(src: String) -> Result<Vec<u32>, String> {
    let mut instructions = Vec::new();

    let token_lines = match tokenize(&src) {
        Ok(token_lines) => token_lines,
        Err(err) => return Err(err.to_string()),
    };

    let labels = extract_labels(&token_lines);

    for tokens in &token_lines {
        let line = tokens[0].line;

        if let Label(_) = tokens[0].variant {
            continue;
        }

        let variants = tokens.iter().map(|token| token.variant.clone()).collect::<Vec<_>>();
        let ambiguous = variants.iter().map(|variant| variant.try_into().expect("'Unknown' token type :(")).collect::<Vec<_>>();

        let pattern = match grammar::find_matching_pattern(&ambiguous) {
            None => return Err(AssemblyError { line, variant: AssemblyErrorVariant::UnknownTokenPattern }.to_string()),
            Some(pattern) => pattern,
        };

        match grammar::construct_instruction(&variants, &pattern.bit_pattern, &pattern.encoding, &labels) {
            Ok(instruction) => instructions.push(instruction),
            Err(error) => return Err(error),
        };
    }

    Ok(instructions)
}

fn extract_labels(token_lines: &Vec<Vec<Token>>) -> HashMap<String, usize> {
    let mut labels = HashMap::new();

    for tokens in token_lines {
        if let Label(name) = &tokens[0].variant {
            labels.insert(name.to_owned(), tokens[0].line);
        }
    }

    labels
}
