use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
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

impl FromStr for Opcode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "nop"    => Ok(Opcode::Nop),
            "add"    => Ok(Opcode::Add),
            "sub"    => Ok(Opcode::Subtract),
            "mul"    => Ok(Opcode::Multiply),
            "div"    => Ok(Opcode::Divide),
            "sdiv"   => Ok(Opcode::DivideSigned),
            "and"    => Ok(Opcode::And),
            "or"     => Ok(Opcode::Or),
            "xor"    => Ok(Opcode::Xor),
            "nand"   => Ok(Opcode::Nand),
            "nor"    => Ok(Opcode::Nor),
            "xnor"   => Ok(Opcode::Xnor),
            "not"    => Ok(Opcode::Not),
            "rsh"    => Ok(Opcode::RightShift),
            "lsh"    => Ok(Opcode::LeftShift),
            "rrol"   => Ok(Opcode::RightRoll),
            "lroll"  => Ok(Opcode::LeftRoll),
            "mov"    => Ok(Opcode::Move),
            "ldi"    => Ok(Opcode::LoadImmediate),
            "ldr"    => Ok(Opcode::LoadRegister),
            "str"    => Ok(Opcode::StoreRegister),
            "push"   => Ok(Opcode::Push),
            "pop"    => Ok(Opcode::Pop),
            "cmp"    => Ok(Opcode::Compare),
            "fcmp"   => Ok(Opcode::CompareFloat),
            "dcmp"   => Ok(Opcode::CompareDouble),
            "b"      => Ok(Opcode::Branch),
            "bg"     => Ok(Opcode::BranchGreater),
            "be"     => Ok(Opcode::BranchEqual),
            "bs"     => Ok(Opcode::BranchSmaller),
            "bge"    => Ok(Opcode::BranchGreaterEqual),
            "bne"    => Ok(Opcode::BranchNotEqual),
            "bse"    => Ok(Opcode::BranchSmallerEqual),
            "immtof" => Ok(Opcode::ImmediateToFloat),
            "immtod" => Ok(Opcode::ImmediateToDouble),
            "itof"   => Ok(Opcode::IntegerToFloat),
            "itod"   => Ok(Opcode::IntegerToDouble),
            "ftoi"   => Ok(Opcode::FloatToInteger),
            "ftod"   => Ok(Opcode::FloatToDouble),
            "dtoi"   => Ok(Opcode::DoubleToInteger),
            "dtof"   => Ok(Opcode::DoubleToFloat),
            _ => Err(format!("Unknown opcode: '{}'", value)),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Opcode::Nop               => "nop",
            Opcode::Add               => "add",
            Opcode::Subtract          => "sub",
            Opcode::Multiply          => "mul",
            Opcode::Divide            => "div",
            Opcode::DivideSigned      => "sdiv",
            Opcode::And               => "and",
            Opcode::Or                => "or",
            Opcode::Xor               => "xor",
            Opcode::Nand              => "nand",
            Opcode::Nor               => "nor",
            Opcode::Xnor              => "xnor",
            Opcode::Not               => "not",
            Opcode::RightShift        => "rsh",
            Opcode::LeftShift         => "lsh",
            Opcode::RightRoll         => "rrol",
            Opcode::LeftRoll          => "lroll",
            Opcode::Move              => "mov",
            Opcode::LoadImmediate     => "ldi",
            Opcode::LoadRegister      => "ldr",
            Opcode::StoreRegister     => "str",
            Opcode::Push              => "push",
            Opcode::Pop               => "pop",
            Opcode::Compare           => "cmp",
            Opcode::CompareFloat      => "fcmp",
            Opcode::CompareDouble     => "dcmp",
            Opcode::Branch            => "b",
            Opcode::BranchGreater     => "bg",
            Opcode::BranchEqual       => "be",
            Opcode::BranchSmaller     => "bs",
            Opcode::BranchGreaterEqual => "bge",
            Opcode::BranchNotEqual => "bne",
            Opcode::BranchSmallerEqual => "bse",
            Opcode::ImmediateToFloat  => "immtof",
            Opcode::ImmediateToDouble => "immtod",
            Opcode::IntegerToFloat    => "itof",
            Opcode::IntegerToDouble   => "itod",
            Opcode::FloatToInteger    => "ftoi",
            Opcode::FloatToDouble     => "ftod",
            Opcode::DoubleToInteger   => "dtoi",
            Opcode::DoubleToFloat     => "dtof",
        };
        write!(f, "{}", s)
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
