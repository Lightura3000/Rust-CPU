use crate::assembler::tokenization::tokenization_error::TokenizationError;
use std::fmt::Display;

#[derive(Debug)]
pub struct AssemblyError {
    pub line: usize,
    pub column: Option<usize>,
    pub variant: AssemblyErrorVariant
}

#[derive(Debug)]
pub enum AssemblyErrorVariant {
    ImmediateTooLarge { max: u16, got: u16 },
    NoLabelFound { name: String },
    OffsetTooLarge { limit: i32, required: i64 },
    UnknownTokenPattern,
    UnrecognizableParam { param_index: usize },
    TokenizationError(TokenizationError)
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.variant {
            AssemblyErrorVariant::ImmediateTooLarge { max, got } => format!("Immediate is too large. Maximum is {} but got {}", max, got),
            AssemblyErrorVariant::NoLabelFound { name } => format!("No label named {} found", name),
            AssemblyErrorVariant::OffsetTooLarge { limit, required } => format!("Offset is too large. Required {} but limit is {}", required, limit),
            AssemblyErrorVariant::UnknownTokenPattern => "Unknown token pattern".to_string(),
            AssemblyErrorVariant::UnrecognizableParam { param_index: near_idx } => format!("Parameter {} can't be tokenized", *near_idx + 1),
            AssemblyErrorVariant::TokenizationError(err) => err.to_string(),
        };

        let detailed = if let Some(position) = self.column {
            format!("Line {} position {}: {}", self.line + 1, position, str)
        } else {
            format!("Line {}: {}", self.line + 1, str)
        };

        write!(f, "{}", detailed)
    }
}

impl From<TokenizationError> for AssemblyError {
    fn from(error: TokenizationError) -> Self {
        Self {
            line: error.line,
            column: Some(error.position),
            variant: AssemblyErrorVariant::TokenizationError(error),
        }
    }
}
