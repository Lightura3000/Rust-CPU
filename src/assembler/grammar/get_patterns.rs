use std::str::FromStr;
use crate::assembler::grammar::token_pattern::AmbiguousToken::Signed;
use super::{
    encoding::Encoding,
    bit_run_length_coding::BitRunLengthCoding,
    token_pattern::AmbiguousToken::*,
    token_pattern::TokenPattern,
    super::types::opcode::Opcode as Opc,
};

pub fn get_patterns() -> Vec<TokenPattern> {
    let mut patterns = Vec::new();
    patterns.append(&mut nop_patterns());
    patterns.append(&mut add_patterns());
    patterns.append(&mut jump_patterns());
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
        TokenPattern { // Register addition
            expected_tokens: vec![Opcode(Opc::Add), Register, Register, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB CCCC 0000 0000 0000 0000").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('C', 3)]),
        },
        TokenPattern { // Immediate addition
            expected_tokens: vec![Opcode(Opc::Add), Register, Register, Unsigned],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 0001").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
    ]
}

fn sub_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern { // Register subtraction
            expected_tokens: vec![Opcode(Opc::Subtract), Register, Register, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB CCCC 0000 0000 0000 0010").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('C', 3)]),
        },
        TokenPattern { // Immediate subtraction
            expected_tokens: vec![Opcode(Opc::Subtract), Register, Register, Unsigned],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 0011").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
        TokenPattern { // Reverse immediate subtraction
            expected_tokens: vec![Opcode(Opc::Subtract), Register, Unsigned, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 0100").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 3), ('I', 2)]),
        },
    ]
}

fn mul_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern { // Register multiplication
            expected_tokens: vec![Opcode(Opc::Multiply), Register, Register, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB CCCC 0000 0000 0000 0101").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('C', 3)]),
        },
        TokenPattern { // Immediate multiplication
            expected_tokens: vec![Opcode(Opc::Multiply), Register, Register, Unsigned],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 0110").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
    ]
}

fn div_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern { // Unsigned register division
            expected_tokens: vec![Opcode(Opc::Divide), Register, Register, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB CCCC 0000 0000 0000 0111").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('C', 3)]),
        },
        TokenPattern { // Unsigned immediate division
            expected_tokens: vec![Opcode(Opc::Divide), Register, Register, Unsigned],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 1000").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
        TokenPattern { // Unsigned reverse immediate division
            expected_tokens: vec![Opcode(Opc::Divide), Register, Unsigned, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 1001").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
    ]
}

fn sdiv_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern { // Signed register division
            expected_tokens: vec![Opcode(Opc::Divide), Register, Register, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB CCCC 0000 0000 0000 1010").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('C', 3)]),
        },
        TokenPattern { // Signed immediate division
            expected_tokens: vec![Opcode(Opc::Divide), Register, Register, Signed],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 1011").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
        TokenPattern { // Signed reverse immediate division
            expected_tokens: vec![Opcode(Opc::Divide), Register, Signed, Register],
            bit_pattern: BitRunLengthCoding::from_str("0001 AAAA BBBB IIII IIII IIII IIII 1100").unwrap(),
            encoding: Encoding::new(vec![('A', 1), ('B', 2), ('I', 3)]),
        },
    ]
}

fn jump_patterns() -> Vec<TokenPattern> {
    vec![
        TokenPattern {
            expected_tokens: vec![Opcode(Opc::Jump), Label],
            bit_pattern: BitRunLengthCoding::from_str("0110 IIII IIII IIII IIII 0000 0000 0001").unwrap(),
            encoding: Encoding::new(vec![('I', 1)]),
        }
    ]
}
