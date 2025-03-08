use crate::assembler::{
    grammar::bit_push::BitPush,
    grammar::bit_run_length_coding::BitRunLengthCoding,
    grammar::get_patterns::get_patterns,
    grammar::token_pattern::AmbiguousToken,
    grammar::token_pattern::TokenPattern,
    grammar::encoding::Encoding,
    assembly_error::{AssemblyError, AssemblyErrorVariant},
    tokenization::token::{Token, TokenVariant},
};
use std::collections::HashMap;

pub fn find_matching_pattern(tokens: &[AmbiguousToken]) -> Option<TokenPattern> {
    let matching_patterns = get_patterns().into_iter().filter(|p| p.matches(tokens)).collect::<Vec<_>>();
    match matching_patterns.len() {
        0 => None,
        1 => Some(matching_patterns[0].clone()),
        _ => panic!("Found {} matching patterns: {:?}", matching_patterns.len(), matching_patterns),
    }
}

/// Attemps to construct an assembled instruction from tokens
pub fn construct_instruction(tokens: &[Token], bit_pattern: &BitRunLengthCoding, encoding: &Encoding, labels: &HashMap<String, usize>, instruction: usize) -> Result<u32, AssemblyError> {
    let mut bit_push = BitPush::new();

    for &(ch, count) in bit_pattern.get() {
        if ch != '0' && ch != '1' && !ch.is_ascii_uppercase() {
            panic!("Unexpected character '{}'", ch);
        }

        if ch == '0' {
            bit_push.push_zeros(count);
            continue;
        } else if ch == '1' {
            bit_push.push_ones(count);
            continue;
        }

        let token = match tokens.get(*encoding.get(ch).unwrap()) {
            None => panic!("Invalid index"),
            Some(token) => &token.variant,
        };

        // ch is in 'A'..='Z'
        match token {
            TokenVariant::Opcode(opc) => panic!("Opcodes can't be assembled (opcode: '{:?}')", opc),
            TokenVariant::Label(name) => {
                let label_i = match labels.get(name.as_str()) {
                    None => return Err(AssemblyError { line: instruction, variant: AssemblyErrorVariant::NoLabelFound { name: name.to_string() }}),
                    Some(i) => *i as u32 as i32,
                };

                let offset = label_i - instruction as i32;

                let offset = match offset {
                    o if o < i16::MIN as i32 => return Err(AssemblyError { line: instruction, variant: AssemblyErrorVariant::OffsetTooLarge { limit: i16::MIN as i32, required: o as i64 } }),
                    o if o > i16::MAX as i32 => return Err(AssemblyError { line: instruction, variant: AssemblyErrorVariant::OffsetTooLarge { limit: i16::MAX as i32, required: o as i64 } }),
                    _ => offset as u16,
                };

                bit_push.push(offset as u32, count);
            },
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
    use super::super::super::types::opcode::Opcode as Opc;
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
}
