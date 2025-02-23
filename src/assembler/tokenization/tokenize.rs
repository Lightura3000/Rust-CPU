use crate::assembler::{
    assembly_error::{AssemblyError, AssemblyErrorVariant},
    types::token::{Token, TokenVariant}
};

pub fn tokenize(src: &str) -> Result<Vec<Vec<Token>>, AssemblyError> {
    let mut tokens = Vec::new();

    for (line, content) in src.lines().enumerate() {
        let mut line_tokens = Vec::new();

        for (param_index, split) in content.split_whitespace().enumerate() {
            let variant = match TokenVariant::try_from(split) {
                Ok(variant) => variant,
                Err(_) => return Err(AssemblyError {
                    line,
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
