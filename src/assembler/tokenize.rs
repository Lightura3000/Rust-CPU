use crate::assembler::assembly_error::{AssemblyError, AssemblyErrorVariant};
use super::types::token::{Token, TokenVariant};

fn split_with_char_indices(input: &str) -> impl Iterator<Item = (usize, &str)> {
    input.split_whitespace().map(|word| {
        let index = input.find(word).unwrap();
        (index, word)
    })
}

pub fn tokenize(src: &str) -> Result<Vec<Vec<Token>>, AssemblyError> {
    let mut tokens = Vec::new();

    for (line, content) in src.lines().enumerate() {
        let mut line_tokens = Vec::new();

        for (index, split) in split_with_char_indices(content) {
            let variant = match TokenVariant::try_from(split) {
                Ok(variant) => variant,
                Err(_) => return Err(AssemblyError {
                    line,
                    variant: AssemblyErrorVariant::UnrecognizableParam,
                }),
            };

            line_tokens.push(Token {
                line,
                variant,
                range: index..(index + split.len()),
            });
        }

        if !line_tokens.is_empty() {
            tokens.push(line_tokens);
        }
    }

    Ok(tokens)
}
