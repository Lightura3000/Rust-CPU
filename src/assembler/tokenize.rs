use crate::assembler::token::Token;

pub fn tokenize(src: &String) -> Vec<Vec<Token>> {
    let mut tokens = Vec::new();

    for (line, content) in src.lines().enumerate() {
        let mut line_tokens = Vec::new();

        for split in content.split_whitespace() {
            match Token::try_construct(line, split) {
                Some(token) => line_tokens.push(token),
                None => break,
            }
        }

        if line_tokens.len() > 0 {
            tokens.push(line_tokens);
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::assembler::tokenize::tokenize;

    #[test]
    fn test_tokenize() {
        println!("{:?}", tokenize(&"ldi r0 4".to_string()));
    }

    #[test]
    fn test_load_file() {
        tokenize(&std::fs::read_to_string("scripts/fibonacci").unwrap()).iter().for_each(|token| println!("{:?}", token));
    }
}
