use super::encoding::Encoding;
use super::bit_run_length_coding::BitRunLengthCoding;
use crate::assembler::types::opcode::Opcode;
use crate::assembler::tokenization::token::TokenVariant;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenPattern {
    pub expected_tokens: Vec<AmbiguousToken>,
    pub bit_pattern: BitRunLengthCoding,
    pub encoding: Encoding,
}

impl TokenPattern {
    pub fn matches(&self, tokens: &[AmbiguousToken]) -> bool {
        if tokens.len() != self.expected_tokens.len() {
            return false;
        }

        tokens.iter()
            .zip(&self.expected_tokens)
            .all(|(a, b)| a == b)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AmbiguousToken {
    Opcode(Opcode),
    Register,
    Unsigned, // Unsigned 16-bit immediate
    Signed,   // Signed 16-bit immediate
    Label,    // For branch targets
    Bool,     // For signed/unsigned comparisons
}

impl From<&TokenVariant> for AmbiguousToken {
    fn from(value: &TokenVariant) -> Self {
        match value {
            TokenVariant::Opcode(opc) => AmbiguousToken::Opcode(*opc),
            TokenVariant::Label(_) => AmbiguousToken::Label,
            TokenVariant::Unsigned(_) => AmbiguousToken::Unsigned,
            TokenVariant::Signed(_) => AmbiguousToken::Signed,
            TokenVariant::Register(_) => AmbiguousToken::Register,
            TokenVariant::Bool(_) => AmbiguousToken::Bool,
        }
    }
}

impl From<TokenVariant> for AmbiguousToken {
    fn from(val: TokenVariant) -> Self {
        match val {
            TokenVariant::Opcode(opc) => AmbiguousToken::Opcode(opc),
            TokenVariant::Label(_) => AmbiguousToken::Label,
            TokenVariant::Unsigned(_) => AmbiguousToken::Unsigned,
            TokenVariant::Signed(_) => AmbiguousToken::Signed,
            TokenVariant::Register(_) => AmbiguousToken::Register,
            TokenVariant::Bool(_) => AmbiguousToken::Bool,
        }
    }
}
