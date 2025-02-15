use super::types::token::Token;

fn split_with_char_indices(input: &str) -> impl Iterator<Item = (usize, &str)> {
    input.split_whitespace().map(|word| {
        let index = input.find(word).unwrap();
        (index, word)
    })
}

pub fn tokenize(src: &str) -> Vec<Vec<Token>> {
    let mut tokens = Vec::new();

    for (line, content) in src.lines().enumerate() {
        let mut line_tokens = Vec::new();

        for (index, split) in split_with_char_indices(content) {
            line_tokens.push(Token {
                line,
                variant: split.into(),
                range: index..(index + split.len()),
            });
        }

        if !line_tokens.is_empty() {
            tokens.push(line_tokens);
        }
    }

    tokens
}
