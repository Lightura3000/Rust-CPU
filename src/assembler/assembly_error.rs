use std::fmt::Display;
use std::ops::Range;

pub struct AssemblyError {
    pub line: usize,
    pub variant: AssemblyErrorVariant
}

pub enum AssemblyErrorVariant {
    NeedsStartingOpcode,
    ParamAmount { expected: usize, got: usize },
    ParamTypes,
    WrongParamType { range: Range<usize>, msg: String },
    ImmediateTooLarge { max: u16, got: u16 },
    NoLabelFound { name: String },
    OffsetTooLarge { limit: i32, required: i64 },
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.variant {
            AssemblyErrorVariant::NeedsStartingOpcode => "Line needs to start with an opcode".to_string(),
            AssemblyErrorVariant::ParamAmount { expected, got } => format!("Expected {} parameters, got {}", expected, got),
            AssemblyErrorVariant::ParamTypes => "Arguments have invalid types".to_string(),
            AssemblyErrorVariant::WrongParamType { range, msg } => format!("range: {:?} msg: {}", range, msg),
            AssemblyErrorVariant::ImmediateTooLarge { max, got } => format!("Immediate is too large. Maximum is {} but got {}", max, got),
            AssemblyErrorVariant::NoLabelFound { name } => format!("No label named {} found", name),
            AssemblyErrorVariant::OffsetTooLarge { limit, required } => format!("Offset is too large. Required {} but limit is {}", required, limit),
        };
        write!(f, "{}", format!("Line {}: {}", self.line + 1, str))
    }
}
