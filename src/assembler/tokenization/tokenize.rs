use crate::assembler::{
    tokenization::{
        provider::{TokenProviderResponse, TokenVariantProvider},
        uint_provider::UIntProvider
    },
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

/// Simulate feeding characters from `chars` starting at `start_idx`
/// into the given provider. If the provider finishes a token, return
/// `Some((token_variant, new_index))`, where `new_index` is the index
/// after the consumed token. Otherwise, return `None`.
fn simulate_provider(
    mut provider: Box<dyn TokenVariantProvider>,
    chars: &[char],
    start_idx: usize,
) -> Option<(TokenVariant, usize)> {
    let mut index = start_idx;

    loop {
        if index < chars.len() {
            let c = chars[index];
            match provider.give_char(c) {
                TokenProviderResponse::CharProcessedSuccessfully => index += 1,
                TokenProviderResponse::TokenFinished(token_variant) => {
                    // We assume the finishing character is consumed.
                    return Some((token_variant, index + 1));
                }
                TokenProviderResponse::InvalidCharNoResult => return None,
            }
        } else {
            // End-of-line: force token finish by sending a dummy character.
            let provider_response = provider.give_char('\0');
            return match provider_response {
                TokenProviderResponse::TokenFinished(token_variant) => Some((token_variant, index)),
                _ => None,
            };
        }
    }
}

/// Tokenize the source code. For each line, the function first skips any
/// whitespace until it encounters a non-whitespace character. Then it tries all
/// available provider factories, starting at that position. If a provider accepts
/// one or more characters but then fails, we "roll back" to the start index and
/// try the next provider. If none can produce a token, the function panics.
pub fn tokenize_new(src: &str) -> Result<Vec<Vec<Token>>, AssemblyError> {
    // List of provider factories. Each returns a fresh provider boxed as a trait object.
    let provider_factories: Vec<fn() -> Box<dyn TokenVariantProvider>> = vec![
        || Box::new(UIntProvider::new()),
        // Additional provider factories can be added here.
    ];

    let mut tokens: Vec<Vec<Token>> = Vec::new();

    for (line_no, line) in src.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut line_tokens: Vec<Token> = Vec::new();
        let mut token_start = 0;

        while token_start < chars.len() {
            // Skip any whitespace until a non-whitespace character is found.
            while token_start < chars.len() && chars[token_start].is_whitespace() {
                token_start += 1;
            }
            if token_start >= chars.len() {
                break;
            }

            let mut token_found = false;
            // Try each provider candidate starting at `token_start`
            for factory in &provider_factories {
                let provider = factory();

                if let Some((token_variant, new_index)) =
                    simulate_provider(provider, &chars, token_start)
                {
                    line_tokens.push(Token {
                        line: line_no,
                        variant: token_variant,
                        // Optionally, include a range if desired, e.g. token_start..new_index,
                    });
                    token_start = new_index;
                    token_found = true;
                    break;
                }
            }
            if !token_found {
                panic!(
                    "No provider accepted input starting at index {} on line {}",
                    token_start, line_no
                );
            }
        }
        tokens.push(line_tokens);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_new() {
        let src = "1234 0b1511";
        let tokens = tokenize_new(src);
        println!("{:?}", tokens);
    }
}
