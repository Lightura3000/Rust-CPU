use std::str::FromStr;
use super::{
    encoding::Encoding,
    bit_run_length_coding::BitRunLengthCoding,
    token_pattern::AmbiguousToken::{Opcode, Register, Unsigned},
    token_pattern::TokenPattern,
    super::types::opcode::Opcode as Opc,
};

pub fn get_patterns() -> Vec<TokenPattern> {
    let mut patterns = Vec::new();
    patterns.append(&mut nop_patterns());
    patterns.append(&mut add_patterns());
    patterns
}

fn nop_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern {
            expected_tokens: vec![Opcode(Opc::Nop)],
            bit_pattern: BitRunLengthCoding::from_str("0000 0000 0000 0000 0000 0000 0000 0000").unwrap(),
            encoding: Encoding::new(vec![]),
        },
    ]
}

fn add_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern {
            expected_tokens: vec![Opcode(Opc::Add), Register, Register, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB CCCC 0000 0000 0000 0000").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('C', 3)]),
        },
        TokenPattern {
            expected_tokens: vec![Opcode(Opc::Add), Register, Register, Unsigned],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 0001").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
        TokenPattern {
            expected_tokens: vec![Opcode(Opc::Add), Register, Unsigned, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 0001").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 3), ('I', 2)]),
        },
    ]
}
