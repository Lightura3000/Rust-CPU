use std::fmt::Display;

pub struct AssemblyError {
    pub line: usize,
    pub variant: AssemblyErrorVariant
}

pub enum AssemblyErrorVariant {
    NeedsStartingOpcode,
    ParamAmount { expected: usize, got: usize },
    ParamTypes,
    ImmediateTooLarge { max: u16, got: u16 },
    NoLabelFound { name: String },
    OffsetTooLarge { limit: i32, required: i64 },
    UnknownTokenPattern,
    UnrecognizableParam,
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.variant {
            AssemblyErrorVariant::NeedsStartingOpcode => "Line needs to start with an opcode".to_string(),
            AssemblyErrorVariant::ParamAmount { expected, got } => format!("Expected {} parameters, got {}", expected, got),
            AssemblyErrorVariant::ParamTypes => "Arguments have invalid types".to_string(),
            AssemblyErrorVariant::ImmediateTooLarge { max, got } => format!("Immediate is too large. Maximum is {} but got {}", max, got),
            AssemblyErrorVariant::NoLabelFound { name } => format!("No label named {} found", name),
            AssemblyErrorVariant::OffsetTooLarge { limit, required } => format!("Offset is too large. Required {} but limit is {}", required, limit),
            AssemblyErrorVariant::UnknownTokenPattern => "Unknown token pattern".to_string(),
            AssemblyErrorVariant::UnrecognizableParam => "Some parameter can't be tokenized".to_string(),
        };
        let line_added = format!("Line {}: {}", self.line + 1, str);
        write!(f, "{}", line_added)
    }
}
