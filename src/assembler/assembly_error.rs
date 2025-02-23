use std::fmt::Display;

#[derive(Debug)]
pub struct AssemblyError {
    pub line: usize,
    pub variant: AssemblyErrorVariant
}

#[derive(Debug)]
pub enum AssemblyErrorVariant {
    ImmediateTooLarge { max: u16, got: u16 },
    NoLabelFound { name: String },
    OffsetTooLarge { limit: i32, required: i64 },
    UnknownTokenPattern,
    UnrecognizableParam,
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.variant {
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
