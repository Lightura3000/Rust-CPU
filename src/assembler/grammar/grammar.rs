use std::collections::HashMap;
use super::{
    bit_push::BitPush,
    bit_run_length_coding::BitRunLengthCoding,
    get_patterns::get_patterns,
    token_pattern::{AmbiguousToken, TokenPattern},
};
use crate::assembler::grammar::encoding::Encoding;
use crate::assembler::types::token::TokenVariant;

pub fn find_matching_pattern(tokens: &[AmbiguousToken]) -> Option<TokenPattern> {
    let matching_patterns = get_patterns().into_iter().filter(|p| p.matches(tokens)).collect::<Vec<_>>();
    match matching_patterns.len() {
        0 => None,
        1 => Some(matching_patterns[0].clone()),
        _ => panic!("Found {} matching patterns: {:?}", matching_patterns.len(), matching_patterns),
    }
}

/// Attemps to construct an assembled instruction from tokens
pub fn construct_instruction(tokens: &[TokenVariant], bit_pattern: &BitRunLengthCoding, encoding: &Encoding, labels: &HashMap<String, usize>) -> Result<u32, String> {
    let mut bit_push = BitPush::new();

    for &(ch, count) in bit_pattern.get() {
        if ch != '0' && ch != '1' && !ch.is_ascii_uppercase() {
            return Err(format!("Unexpected character '{}'", ch));
        }

        if ch == '0' {
            bit_push.push_zeros(count);
            continue;
        } else if ch == '1' {
            bit_push.push_ones(count);
            continue;
        }

        let token = match tokens.get(*encoding.get(ch).unwrap()) {
            None => return Err("Invalid index".to_string()),
            Some(token) => token,
        };

        // ch is in 'A'..='Z'
        match token {
            TokenVariant::Opcode(opc) => return Err(format!("Opcodes can't be assembled (opcode: '{:?}')", opc)),
            TokenVariant::Label(_) => todo!(),
            TokenVariant::Unsigned(value) => bit_push.push(*value as u32, count),
            TokenVariant::Signed(value) => bit_push.push(*value as u16 as u32, count),
            TokenVariant::Register(reg) => bit_push.push(*reg as u32, count),
            TokenVariant::Bool(b) => bit_push.push(*b as u32, count),
        }
    }

    Ok(bit_push.get_value().expect("Something went wrong"))
}

#[cfg(test)]
mod tests {
    use super::super::super::types::{register::Register, opcode::Opcode as Opc};
    use super::*;
    use AmbiguousToken::*;

    #[test]
    fn test_nop() {
        assert!(find_matching_pattern(&[Opcode(Opc::Nop)]).is_some());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Register]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Unsigned]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Unsigned]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Unsigned]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Unsigned]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Unsigned]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Label]).is_none());
        assert!(find_matching_pattern(&[Opcode(Opc::Nop), Bool]).is_none());
    }

    #[test]
    fn test_add() {
        assert!(find_matching_pattern(&[Opcode(Opc::Add), Register, Register, Register]).is_some());
        assert!(find_matching_pattern(&[Opcode(Opc::Add), Register, Register, Unsigned]).is_some());
        assert!(find_matching_pattern(&[Opcode(Opc::Add)]).is_none());
    }

    #[test]
    fn test_construct_instruction() {
        let tokens = [
            TokenVariant::Opcode(Opc::Add),
            TokenVariant::Register(Register::R0),
            TokenVariant::Register(Register::R1),
            TokenVariant::Unsigned(0xFFFF),
        ];

        let transformed: Vec<AmbiguousToken> = tokens.iter()
            .map(|token| token.clone().try_into().unwrap())
            .collect();

        let pattern = match find_matching_pattern(&transformed) {
            None => panic!("No matching pattern found"),
            Some(pattern) => pattern,
        };

        println!("{:#08x}", construct_instruction(&tokens, &pattern.bit_pattern, &pattern.encoding, &HashMap::new()).unwrap());
    }
}
