use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Opcode {
    Nop,
    Add,
    Subtract,
    Multiply,
    Divide,
    DivideSigned,
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,
    Not,
    RightShift,
    LeftShift,
    RightRoll,
    LeftRoll,
    Move,
    LoadImmediate,
    LoadRegister,
    StoreRegister,
    Push,
    Pop,
    Compare,
    CompareFloat,
    CompareDouble,
    Branch,
    BranchGreater,
    BranchEqual,
    BranchSmaller,
    BranchGreaterEqual,
    BranchNotEqual,
    BranchSmallerEqual,
    ImmediateToFloat,
    ImmediateToDouble,
    IntegerToFloat,
    IntegerToDouble,
    FloatToInteger,
    FloatToDouble,
    DoubleToInteger,
    DoubleToFloat,
}

/// A static lookup table of `(Opcode, &str)` pairs.
/// This is the *single* source of truth for how `Opcode`
/// variants map to human-readable strings and vice versa.
static OPCODE_TABLE: &[(Opcode, &str)] = &[
    (Opcode::Nop,               "nop"),
    (Opcode::Add,               "add"),
    (Opcode::Subtract,          "sub"),
    (Opcode::Multiply,          "mul"),
    (Opcode::Divide,            "div"),
    (Opcode::DivideSigned,      "sdiv"),
    (Opcode::And,               "and"),
    (Opcode::Or,                "or"),
    (Opcode::Xor,               "xor"),
    (Opcode::Nand,              "nand"),
    (Opcode::Nor,               "nor"),
    (Opcode::Xnor,              "xnor"),
    (Opcode::Not,               "not"),
    (Opcode::RightShift,        "rsh"),
    (Opcode::LeftShift,         "lsh"),
    (Opcode::RightRoll,         "rrol"),
    (Opcode::LeftRoll,          "lroll"),
    (Opcode::Move,              "mov"),
    (Opcode::LoadImmediate,     "ldi"),
    (Opcode::LoadRegister,      "ldr"),
    (Opcode::StoreRegister,     "str"),
    (Opcode::Push,              "push"),
    (Opcode::Pop,               "pop"),
    (Opcode::Compare,           "cmp"),
    (Opcode::CompareFloat,      "fcmp"),
    (Opcode::CompareDouble,     "dcmp"),
    (Opcode::Branch,            "b"),
    (Opcode::BranchGreater,     "bg"),
    (Opcode::BranchEqual,       "be"),
    (Opcode::BranchSmaller,     "bs"),
    (Opcode::BranchGreaterEqual,"bge"),
    (Opcode::BranchNotEqual,    "bne"),
    (Opcode::BranchSmallerEqual,"bse"),
    (Opcode::ImmediateToFloat,  "immtof"),
    (Opcode::ImmediateToDouble, "immtod"),
    (Opcode::IntegerToFloat,    "itof"),
    (Opcode::IntegerToDouble,   "itod"),
    (Opcode::FloatToInteger,    "ftoi"),
    (Opcode::FloatToDouble,     "ftod"),
    (Opcode::DoubleToInteger,   "dtoi"),
    (Opcode::DoubleToFloat,     "dtof"),
];

impl FromStr for Opcode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // Look up the string in our table.
        // If found, return the associated `Opcode`.
        // Otherwise, return an error.
        OPCODE_TABLE
            .iter()
            .find_map(|(opcode, name)| {
                if *name == value {
                    Some(*opcode)
                } else {
                    None
                }
            })
            .ok_or_else(|| format!("Unknown opcode: '{}'", value))
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Find `self` in the table and write out its string.
        // We use `unreachable!()` if somehow we fail to find it,
        // because our enum and static table should match exactly.
        for (opcode, name) in OPCODE_TABLE {
            if opcode == self {
                return write!(f, "{}", name);
            }
        }
        unreachable!("No corresponding opcode found: {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_try_from_ok() {
        assert_eq!(Opcode::try_from("nop").unwrap(), Opcode::Nop);
        assert_eq!(Opcode::try_from("add").unwrap(), Opcode::Add);
        assert_eq!(Opcode::try_from("dtof").unwrap(), Opcode::DoubleToFloat);
    }

    #[test]
    fn test_try_from_err() {
        assert!(Opcode::try_from("unknown").is_err());
        assert!(Opcode::try_from("").is_err());
    }

    #[test]
    fn test_try_into() {
        let opcode: Result<Opcode, _> = "sub".try_into();
        assert_eq!(opcode.unwrap(), Opcode::Subtract);
    }

    #[test]
    fn test_from_str() {
        let opcode = "mul".parse::<Opcode>().unwrap();
        assert_eq!(opcode, Opcode::Multiply);
    }

    #[test]
    fn test_display() {
        assert_eq!(Opcode::Nop.to_string(), "nop");
        assert_eq!(Opcode::DivideSigned.to_string(), "sdiv");
        assert_eq!(Opcode::RightRoll.to_string(), "rrol");
    }
}
