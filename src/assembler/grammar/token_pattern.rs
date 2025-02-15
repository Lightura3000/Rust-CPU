use super::encoding::Encoding;
use super::bit_run_length_coding::BitRunLengthCoding;
use crate::assembler::types::opcode::Opcode;
use crate::assembler::types::token::TokenVariant;

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

impl TryInto<AmbiguousToken> for TokenVariant {
    type Error = ();

    fn try_into(self) -> Result<AmbiguousToken, Self::Error> {
        match self {
            TokenVariant::Opcode(opc) => Ok(AmbiguousToken::Opcode(opc)),
            TokenVariant::Label(_) => Ok(AmbiguousToken::Label),
            TokenVariant::Unsigned(_) => Ok(AmbiguousToken::Unsigned),
            TokenVariant::Signed(_) => Ok(AmbiguousToken::Signed),
            TokenVariant::Register(_) => Ok(AmbiguousToken::Register),
            TokenVariant::Bool(_) => Ok(AmbiguousToken::Bool),
            TokenVariant::Unknown => Err(()),
        }
    }
}
